use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, HttpMessage, HttpResponse, web};
use chrono::{DateTime, Utc};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json;
use std::future::{Ready, ready};
use stock_application::services::auth_service::Claims;
use stock_domain::entities::user::UserRole;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::AppState;
use crate::services::token_blacklist::TokenInfo;

// Define a struct to hold our authentication configuration
pub struct Authentication {
	pub exclude_paths: Vec<String>,
}

// Implementation of Transform for Authentication
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type InitError = ();
	type Transform = AuthenticationMiddleware<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(AuthenticationMiddleware {
			service,
			exclude_paths: self.exclude_paths.clone(),
		}))
	}
}

// Define the middleware that will check for authentication
pub struct AuthenticationMiddleware<S> {
	service: S,
	exclude_paths: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		// Check if the path is excluded from authentication
		let path = req.path().to_string();
		for excluded_path in &self.exclude_paths {
			if path.starts_with(excluded_path) {
				let future = self.service.call(req);
				return Box::pin(async move {
					let response = future.await?;
					Ok(response.map_into_left_body())
				});
			}
		}

		// Get application state to access JWT secret and blacklist service
		let app_state = req.app_data::<web::Data<AppState>>().cloned();

		// Extract token from cookie
		let auth_cookie = req.cookie("auth_token");
		let auth_token = auth_cookie.map(|c| c.value().to_string());

		// Check if this is an HTMX request
		let is_htmx_request = req.headers().get("HX-Request").is_some();

		// Check if this is an AJAX request
		let is_ajax_request = req
			.headers()
			.get("X-Requested-With")
			.is_some_and(|v| v.to_str().unwrap_or("") == "XMLHttpRequest");

		// Process based on token
		if let Some(token) = auth_token {
			if let Some(app_state) = app_state {
				// Verify token
				match decode::<Claims>(
					&token,
					&DecodingKey::from_secret(app_state.jwt_secret.as_bytes()),
					&Validation::default(),
				) {
					Ok(token_data) => {
						// Check if token is revoked
						if app_state.blacklist_service.is_token_revoked(&token_data.claims.jti) {
							warn!("Attempted use of revoked token with JTI: {}", token_data.claims.jti);
							return Box::pin(
								async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) },
							);
						}

						// Check if token is expired
						let now = Utc::now().timestamp();
						if token_data.claims.exp <= now {
							warn!("Attempted use of expired token with JTI: {}", token_data.claims.jti);
							return Box::pin(
								async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) },
							);
						}

						// Extract user info from token
						let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) else {
							warn!("Invalid user ID in token: {}", token_data.claims.sub);
							return Box::pin(
								async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) },
							);
						};

						// Register token in blacklist service for tracking
						// This helps track active tokens for revocation
						let token_info = TokenInfo {
							jti: token_data.claims.jti.clone(),
							user_id,
							expires_at: DateTime::from_timestamp(token_data.claims.exp, 0)
								.unwrap_or_else(|| Utc::now() + chrono::Duration::hours(24)),
						};
						app_state.blacklist_service.register_token(token_info);

						// Add user info to request extensions
						req.extensions_mut().insert(user_id);

						// Also add user role
						let role = match token_data.claims.role.as_str() {
							"MANAGER" => UserRole::Manager,
							_ => UserRole::Seller,
						};
						req.extensions_mut().insert(role.clone());

						// Add JTI to request extensions for potential future use
						req.extensions_mut().insert(token_data.claims.jti.clone());

						// Add username to request extensions
						req.extensions_mut().insert(token_data.claims.username.clone());

						debug!(
							"Authenticated user: {} (ID: {}, Role: {}, JTI: {})",
							token_data.claims.username, user_id, role, token_data.claims.jti
						);

						// Continue with the request
						let future = self.service.call(req);
						Box::pin(async move {
							let response = future.await?;
							Ok(response.map_into_left_body())
						})
					},
					Err(e) => {
						// Invalid token - log the error and return auth response
						warn!("Invalid JWT token: {}", e);
						Box::pin(async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) })
					},
				}
			} else {
				// Server configuration error - return auth response
				warn!("Missing AppState in authentication middleware");
				Box::pin(async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) })
			}
		} else {
			// No token found - return auth response
			debug!("No authentication token found for path: {}", path);
			Box::pin(async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) })
		}
	}
}

fn create_auth_response<B>(
	req: ServiceRequest,
	is_htmx_request: bool,
	is_ajax_request: bool,
) -> ServiceResponse<EitherBody<B>> {
	let response = if is_htmx_request {
		// For HTMX requests, use HX-Redirect header to trigger client-side redirect
		HttpResponse::Unauthorized()
			.insert_header(("HX-Redirect", "/auth/login"))
			.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
			.insert_header(("Pragma", "no-cache"))
			.insert_header(("Expires", "0"))
			.body("Authentication required")
	} else if is_ajax_request {
		// For AJAX requests, return JSON response with redirect URL
		HttpResponse::Unauthorized()
			.insert_header(("Content-Type", "application/json"))
			.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
			.insert_header(("Pragma", "no-cache"))
			.insert_header(("Expires", "0"))
			.json(serde_json::json!({
				"error": "Authentication required",
				"redirect": "/auth/login"
			}))
	} else {
		// For regular browser requests, return 302 redirect
		HttpResponse::Found()
			.insert_header(("Location", "/auth/login"))
			.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
			.insert_header(("Pragma", "no-cache"))
			.insert_header(("Expires", "0"))
			.finish()
	};

	let (http_req, _) = req.into_parts();
	ServiceResponse::new(http_req, response).map_into_right_body()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::services::token_blacklist::TokenBlacklistService;
	use actix_web::{App, HttpResponse, test, web};
	use chrono::Duration;
	use std::sync::Arc;
	use stock_application::services::auth_service::AuthService;
	use stock_infrastructure::db::establish_connection_pool;
	use stock_infrastructure::repositories::user_repository::DieselUserRepository;

	async fn protected_handler() -> HttpResponse {
		HttpResponse::Ok().body("Protected")
	}

	fn create_test_app_state() -> web::Data<AppState> {
		let pool = establish_connection_pool();
		let pool = Arc::new(pool);
		let user_repo = Arc::new(DieselUserRepository::new(pool));
		let auth_service = Arc::new(AuthService::new(user_repo, "test_secret".to_string()));
		let blacklist_service = Arc::new(TokenBlacklistService::new());

		web::Data::new(AppState {
			category_service: Arc::new(stock_application::CategoryService::new(
				Arc::new(stock_infrastructure::repositories::category_repository::DieselCategoryRepository::new(
					Arc::new(establish_connection_pool())
				))
			)),
			product_service: Arc::new(stock_application::ProductService::new(
				Arc::new(stock_infrastructure::repositories::product_repository::DieselProductRepository::new(
					Arc::new(establish_connection_pool())
				))
			)),
			warehouse_service: Arc::new(stock_application::WarehouseService::new(
				Arc::new(stock_infrastructure::repositories::warehouse_repository::DieselWarehouseRepository::new(
					Arc::new(establish_connection_pool())
				))
			)),
			stock_item_service: Arc::new(stock_application::StockItemService::new(
				Arc::new(stock_infrastructure::repositories::stock_item_repository::DieselStockItemRepository::new(
					Arc::new(establish_connection_pool())
				))
			)),
			transaction_service: Arc::new(stock_application::StockTransactionService::new(
				Arc::new(stock_infrastructure::repositories::stock_transaction_repository::DieselStockTransactionRepository::new(
					Arc::new(establish_connection_pool())
				)),
				Arc::new(stock_infrastructure::repositories::stock_item_repository::DieselStockItemRepository::new(
					Arc::new(establish_connection_pool())
				))
			)),
			auth_service,
			blacklist_service,
			jwt_secret: "test_secret".to_string(),
			enable_registration: true,
		})
	}

	#[actix_web::test]
	async fn test_redirect_for_regular_request() {
		let app_state = create_test_app_state();
		let app = test::init_service(
			App::new()
				.app_data(app_state)
				.wrap(Authentication {
					exclude_paths: vec!["/auth/login".to_string()],
				})
				.route("/protected", web::get().to(protected_handler)),
		)
		.await;

		let req = test::TestRequest::get().uri("/protected").to_request();
		let resp = test::call_service(&app, req).await;

		assert!(
			resp.status() == actix_web::http::StatusCode::FOUND
				|| resp.status() == actix_web::http::StatusCode::UNAUTHORIZED
		);

		if resp.status() == actix_web::http::StatusCode::FOUND {
			assert_eq!(resp.headers().get("Location").unwrap(), "/auth/login");
		}
	}

	#[actix_web::test]
	async fn test_htmx_redirect() {
		let app_state = create_test_app_state();
		let app = test::init_service(
			App::new()
				.app_data(app_state)
				.wrap(Authentication {
					exclude_paths: vec!["/auth/login".to_string()],
				})
				.route("/protected", web::get().to(protected_handler)),
		)
		.await;

		let req = test::TestRequest::get()
			.uri("/protected")
			.insert_header(("HX-Request", "true"))
			.to_request();
		let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
		assert_eq!(resp.headers().get("HX-Redirect").unwrap(), "/auth/login");
	}

	#[actix_web::test]
	async fn test_ajax_redirect() {
		let app_state = create_test_app_state();
		let app = test::init_service(
			App::new()
				.app_data(app_state)
				.wrap(Authentication {
					exclude_paths: vec!["/auth/login".to_string()],
				})
				.route("/protected", web::get().to(protected_handler)),
		)
		.await;

		let req = test::TestRequest::get()
			.uri("/protected")
			.insert_header(("X-Requested-With", "XMLHttpRequest"))
			.to_request();
		let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
		assert_eq!(resp.headers().get("Content-Type").unwrap(), "application/json");
	}

	#[actix_web::test]
	async fn test_excluded_path_access() {
		let app_state = create_test_app_state();
		let app = test::init_service(
			App::new()
				.app_data(app_state)
				.wrap(Authentication {
					exclude_paths: vec!["/auth/login".to_string(), "/public".to_string()],
				})
				.route("/public/health", web::get().to(|| async { "OK" })),
		)
		.await;

		let req = test::TestRequest::get().uri("/public/health").to_request();
		let resp = test::call_service(&app, req).await;

		assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
	}

	#[actix_web::test]
	async fn test_revoked_token_rejection() {
		let app_state = create_test_app_state();

		// Register and immediately revoke a token
		let token_info = TokenInfo {
			jti: "revoked-token".to_string(),
			user_id: uuid::Uuid::new_v4(),
			expires_at: Utc::now() + Duration::hours(1),
		};
		app_state.blacklist_service.register_token(token_info);
		let _ = app_state.blacklist_service.revoke_token("revoked-token");

		let app = test::init_service(
			App::new()
				.app_data(app_state)
				.wrap(Authentication {
					exclude_paths: vec!["/auth/login".to_string()],
				})
				.route("/protected", web::get().to(protected_handler)),
		)
		.await;

		// Create a valid JWT token but with the revoked JTI
		use jsonwebtoken::{EncodingKey, Header, encode};
		let claims = stock_application::services::auth_service::Claims {
			sub: uuid::Uuid::new_v4().to_string(),
			username: "testuser".to_string(),
			role: "MANAGER".to_string(),
			exp: (Utc::now() + Duration::hours(1)).timestamp(),
			iat: Utc::now().timestamp(),
			jti: "revoked-token".to_string(),
		};

		let token = encode(
			&Header::default(),
			&claims,
			&EncodingKey::from_secret("test_secret".as_bytes()),
		)
		.unwrap();

		let req = test::TestRequest::get()
			.uri("/protected")
			.cookie(actix_web::cookie::Cookie::new("auth_token", token))
			.to_request();
		let resp = test::call_service(&app, req).await;

		// Should be rejected due to revoked token
		assert!(
			resp.status() == actix_web::http::StatusCode::UNAUTHORIZED
				|| resp.status() == actix_web::http::StatusCode::FOUND
		);
	}
}

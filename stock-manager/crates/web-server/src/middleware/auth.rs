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
						// Check if token is blacklisted
						if app_state.blacklist_service.is_token_revoked(&token_data.claims.jti) {
							tracing::warn!("Attempted use of revoked token with JTI: {}", token_data.claims.jti);
							return Box::pin(
								async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) },
							);
						}

						// Extract user info from token
						let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) else {
							return Box::pin(
								async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) },
							);
						};

						// Register token in blacklist service if not already known
						// This helps track active tokens for revocation
						let token_info = TokenInfo {
							jti: token_data.claims.jti.clone(),
							user_id,
							expires_at: DateTime::from_timestamp(token_data.claims.exp, 0).unwrap_or_else(Utc::now),
						};
						app_state.blacklist_service.register_token(token_info);

						// Add user info to request extensions
						req.extensions_mut().insert(user_id);

						// Also add user role
						let role = match token_data.claims.role.as_str() {
							"MANAGER" => UserRole::Manager,
							_ => UserRole::Seller,
						};
						req.extensions_mut().insert(role);

						// Add JTI to request extensions for potential future use
						req.extensions_mut().insert(token_data.claims.jti);

						// Continue with the request
						let future = self.service.call(req);
						Box::pin(async move {
							let response = future.await?;
							Ok(response.map_into_left_body())
						})
					},
					Err(e) => {
						// Invalid token - log the error and return auth response
						tracing::warn!("Invalid JWT token: {}", e);
						Box::pin(async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) })
					},
				}
			} else {
				// Server configuration error - return auth response
				tracing::error!("Missing AppState in authentication middleware");
				Box::pin(async move { Ok(create_auth_response(req, is_htmx_request, is_ajax_request)) })
			}
		} else {
			// No token found - return auth response
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
			.body("Authentication required")
	} else if is_ajax_request {
		// For AJAX requests, return JSON response with redirect URL
		HttpResponse::Unauthorized()
			.insert_header(("Content-Type", "application/json"))
			.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
			.json(serde_json::json!({
				"error": "Authentication required",
				"redirect": "/auth/login"
			}))
	} else {
		// For regular browser requests, return 302 redirect
		HttpResponse::Found()
			.insert_header(("Location", "/auth/login"))
			.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
			.finish()
	};

	let (http_req, _) = req.into_parts();
	ServiceResponse::new(http_req, response).map_into_right_body()
}

#[cfg(test)]
mod tests {
	use super::*;
	use actix_web::{App, HttpResponse, test, web};

	async fn protected_handler() -> HttpResponse {
		HttpResponse::Ok().body("Protected")
	}

	#[actix_web::test]
	async fn test_redirect_for_regular_request() {
		let app = test::init_service(
			App::new()
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
		let app = test::init_service(
			App::new()
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
		let app = test::init_service(
			App::new()
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
}

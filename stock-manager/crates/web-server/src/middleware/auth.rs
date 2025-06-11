use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, HttpMessage, HttpResponse, web};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json;
use std::future::{Ready, ready};
use stock_application::services::auth_service::Claims;
use stock_domain::entities::user::UserRole;
use uuid::Uuid;

use crate::AppState;

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
	type Response = ServiceResponse<B>;
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
	type Response = ServiceResponse<B>;
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
					Ok(response)
				});
			}
		}

		// Get application state to access JWT secret
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
						// Extract user info from token
						let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) else {
							return Box::pin(
								async move { Err(create_redirect_error(is_htmx_request, is_ajax_request)) },
							);
						};

						// Add user info to request extensions
						req.extensions_mut().insert(user_id);

						// Also add user role
						let role = match token_data.claims.role.as_str() {
							"MANAGER" => UserRole::Manager,
							_ => UserRole::Seller,
						};
						req.extensions_mut().insert(role);

						// Continue with the request
						let future = self.service.call(req);
						Box::pin(async move {
							let response = future.await?;
							Ok(response)
						})
					},
					Err(_) => {
						// Invalid token - redirect to login
						Box::pin(async move { Err(create_redirect_error(is_htmx_request, is_ajax_request)) })
					},
				}
			} else {
				// Server configuration error - redirect to login
				Box::pin(async move { Err(create_redirect_error(is_htmx_request, is_ajax_request)) })
			}
		} else {
			// No token found - redirect to login
			Box::pin(async move { Err(create_redirect_error(is_htmx_request, is_ajax_request)) })
		}
	}
}

/// Creates an appropriate redirect error based on the request type
fn create_redirect_error(is_htmx_request: bool, is_ajax_request: bool) -> Error {
	if is_htmx_request {
		// For HTMX requests, use HX-Redirect header to trigger client-side redirect
		actix_web::error::InternalError::from_response(
			"Authentication required",
			HttpResponse::Unauthorized()
				.insert_header(("HX-Redirect", "/auth/login"))
				.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
				.body("Authentication required"),
		)
		.into()
	} else if is_ajax_request {
		// For AJAX requests, return JSON response with redirect URL
		actix_web::error::InternalError::from_response(
			"Authentication required",
			HttpResponse::Unauthorized()
				.insert_header(("Content-Type", "application/json"))
				.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
				.json(serde_json::json!({
					"error": "Authentication required",
					"redirect": "/auth/login"
				})),
		)
		.into()
	} else {
		// For regular browser requests, return 302 redirect
		actix_web::error::InternalError::from_response(
			"Authentication required",
			HttpResponse::Found()
				.insert_header(("Location", "/auth/login"))
				.insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))
				.finish(),
		)
		.into()
	}
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

		assert_eq!(resp.status(), actix_web::http::StatusCode::FOUND);
		assert_eq!(resp.headers().get("Location").unwrap(), "/auth/login");
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

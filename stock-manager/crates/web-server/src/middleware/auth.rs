use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpMessage, web};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
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
				let fut = self.service.call(req);
				return Box::pin(async move {
					let res = fut.await?;
					Ok(res)
				});
			}
		}

		// Get application state to access JWT secret
		let app_state = req.app_data::<web::Data<AppState>>().cloned();

		// Extract token from cookie
		let auth_cookie = req.cookie("auth_token");
		let auth_token = auth_cookie.map(|c| c.value().to_string());

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
						let user_id = match Uuid::parse_str(&token_data.claims.sub) {
							Ok(id) => id,
							Err(_) => {
								return Box::pin(async { Err(ErrorUnauthorized("Invalid token")) });
							},
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
						let fut = self.service.call(req);
						return Box::pin(async move {
							let res = fut.await?;
							Ok(res)
						});
					},
					Err(_) => {
						return Box::pin(async { Err(ErrorUnauthorized("Invalid token")) });
					},
				}
			} else {
				return Box::pin(async { Err(ErrorUnauthorized("Server configuration error")) });
			}
		} else {
			// No token found, redirect to login
			return Box::pin(async { Err(ErrorUnauthorized("Authentication required")) });
		}
	}
}

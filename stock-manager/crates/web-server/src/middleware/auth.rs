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
use tracing::{debug, info, warn};
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

						let renewal_threshold = 15 * 60; // 15 minutes in seconds
						let needs_renewal = (token_data.claims.exp - now) < renewal_threshold;

						let mut new_token_info = None;

						if needs_renewal {
							debug!(
								"Token for user {} expires soon, generating renewal",
								token_data.claims.username
							);

							match app_state.auth_service.generate_token_from_claims(&token_data.claims) {
								Ok((new_token, new_jti)) => {
									info!(
										"Auto-renewed token for user {} (old JTI: {}, new JTI: {})",
										token_data.claims.username, token_data.claims.jti, new_jti
									);

									let _ = app_state.blacklist_service.revoke_token(&token_data.claims.jti);

									let new_expires_at = DateTime::from_timestamp(now + 3600, 0)
										.unwrap_or_else(|| Utc::now() + chrono::Duration::hours(1));

									let token_info = TokenInfo {
										jti: new_jti.clone(),
										user_id,
										expires_at: new_expires_at,
									};
									app_state.blacklist_service.register_token(token_info);

									new_token_info = Some((new_token, new_jti));
								},
								Err(e) => {
									warn!(
										"Failed to generate renewal token for user {}: {}",
										token_data.claims.username, e
									);
								},
							}
						} else if !app_state.blacklist_service.is_token_registered(&token_data.claims.jti) {
							let token_info = TokenInfo {
								jti: token_data.claims.jti.clone(),
								user_id,
								expires_at: DateTime::from_timestamp(token_data.claims.exp, 0)
									.unwrap_or_else(|| Utc::now() + chrono::Duration::hours(1)),
							};
							app_state.blacklist_service.register_token(token_info);
							debug!(
								"Re-registered token {} for user {} (recovery)",
								token_data.claims.jti, user_id
							);
						}

						// Add user info to request extensions
						req.extensions_mut().insert(user_id);

						// Also add user role
						let role = match token_data.claims.role.as_str() {
							"MANAGER" => UserRole::Manager,
							_ => UserRole::Seller,
						};
						req.extensions_mut().insert(role.clone());

						// Add JTI to request extensions (use new JTI if renewed)
						let current_jti = new_token_info
							.as_ref()
							.map_or_else(|| token_data.claims.jti.clone(), |(_, jti)| jti.clone());
						req.extensions_mut().insert(current_jti.clone());

						// Add username to request extensions
						req.extensions_mut().insert(token_data.claims.username.clone());

						debug!(
							"Authenticated user: {} (ID: {}, Role: {}, JTI: {}{})",
							token_data.claims.username,
							user_id,
							role,
							current_jti,
							if new_token_info.is_some() { " - RENEWED" } else { "" }
						);

						// Continue with the request
						let future = self.service.call(req);
						Box::pin(async move {
							let mut response = future.await?;

							// If we have a new token, set it as a cookie
							if let Some((new_token, _)) = new_token_info {
								let cookie = actix_web::cookie::Cookie::build("auth_token", new_token)
									.http_only(true)
									.same_site(actix_web::cookie::SameSite::Strict)
									.path("/")
									.max_age(actix_web::cookie::time::Duration::seconds(3600)) // 1 hour
									.finish();

								response.response_mut().add_cookie(&cookie).ok();
								debug!("Set renewed token cookie in response");
							}

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

use actix_web::{HttpMessage, HttpRequest, HttpResponse, Result, web};
use askama::DynTemplate;
use jsonwebtoken::{DecodingKey, Validation, decode};
use stock_application::services::auth_service::{Claims, Credentials};
use tracing::{debug, info, warn};

use crate::AppState;
use crate::dtos::auth::{LoginDto, LoginTemplate, RegisterDto, RegisterTemplate};

pub async fn login_form(data: web::Data<AppState>) -> Result<HttpResponse> {
	let template = LoginTemplate {
		enable_registration: data.enable_registration,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn register_form(data: web::Data<AppState>) -> Result<HttpResponse> {
	let _ = data;
	let template = RegisterTemplate {};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

#[allow(dead_code)]
pub async fn register(state: web::Data<AppState>, form: web::Form<RegisterDto>) -> Result<HttpResponse> {
	let auth_service = &state.auth_service;

	let dto = form.0.into();

	match auth_service.register(dto).await {
		Ok(user) => {
			info!("User registered successfully: {} (ID: {})", user.username, user.id);
			Ok(HttpResponse::Found()
				.append_header(("Location", "/auth/login"))
				.finish())
		},
		Err(e) => {
			warn!("Registration failed: {}", e);
			Ok(HttpResponse::BadRequest().body(format!("Registration failed: {e}")))
		},
	}
}

pub async fn login(state: web::Data<AppState>, form: web::Form<LoginDto>) -> Result<HttpResponse> {
	let auth_service = &state.auth_service;

	let credentials = Credentials {
		username: form.username.clone(),
		password: form.password.clone(),
	};

	match auth_service.login(credentials).await {
		Ok(token) => {
			info!("User logged in successfully: {} (ID: {})", form.username, token.user_id);

			// Set JWT token as a cookie
			Ok(HttpResponse::Found()
				.cookie(
					actix_web::cookie::Cookie::build("auth_token", token.token)
						.http_only(true)
						.same_site(actix_web::cookie::SameSite::Strict)
						.path("/")
						.max_age(actix_web::cookie::time::Duration::seconds(86400)) // 24 hours
						.finish(),
				)
				// Add username cookie for UI display
				.cookie(
					actix_web::cookie::Cookie::build("username", form.username.clone())
						.path("/")
						.max_age(actix_web::cookie::time::Duration::seconds(86400)) // 24 hours
						.finish(),
				)
				.append_header(("Location", "/"))
				.finish())
		},
		Err(e) => {
			warn!("Login failed for user '{}': {}", form.username, e);
			Ok(HttpResponse::BadRequest().body(format!("Login failed: {e}")))
		},
	}
}

pub async fn logout(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse> {
	// Try to revoke the current token if it exists
	if let Some(auth_cookie) = req.cookie("auth_token") {
		let token = auth_cookie.value();

		// Decode the token to extract the JTI for revocation
		match decode::<Claims>(
			token,
			&DecodingKey::from_secret(state.jwt_secret.as_bytes()),
			&Validation::default(),
		) {
			Ok(token_data) => {
				let jti = &token_data.claims.jti;
				let username = &token_data.claims.username;

				// Revoke the token
				if state.blacklist_service.revoke_token(jti) {
					info!("Token revoked successfully for user '{}' (JTI: {})", username, jti);
				} else {
					warn!("Failed to revoke token for user '{}' (JTI: {})", username, jti);
				}

				debug!("User '{}' logged out", username);
			},
			Err(e) => {
				warn!("Failed to decode token during logout: {}", e);
				// Continue with logout even if token decoding fails
			},
		}
	}

	// Clear the auth cookies and redirect
	Ok(HttpResponse::Found()
		.cookie(
			actix_web::cookie::Cookie::build("auth_token", "")
				.http_only(true)
				.same_site(actix_web::cookie::SameSite::Strict)
				.path("/")
				.max_age(actix_web::cookie::time::Duration::seconds(-1))
				.finish(),
		)
		// Also clear username cookie
		.cookie(
			actix_web::cookie::Cookie::build("username", "")
				.path("/")
				.max_age(actix_web::cookie::time::Duration::seconds(-1))
				.finish(),
		)
		.append_header(("Location", "/auth/login"))
		.finish())
}

// Admin endpoint to get token statistics (useful for monitoring)
pub async fn token_stats(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse> {
	// Check if user is a manager (this would typically be done via middleware)
	let is_manager = req
		.extensions()
		.get::<stock_domain::entities::user::UserRole>()
		.is_some_and(|role| *role == stock_domain::entities::user::UserRole::Manager);

	if !is_manager {
		return Ok(HttpResponse::Forbidden().json(serde_json::json!({
			"error": "Manager role required"
		})));
	}

	let stats = state.blacklist_service.get_detailed_stats();

	Ok(HttpResponse::Ok().json(serde_json::json!({
		"active_users": stats.basic.active_users_count,
		"total_active_tokens": stats.basic.total_active_tokens,
		"revoked_tokens": stats.basic.revoked_tokens_count,
		"expired_active_tokens": stats.expired_active_tokens,
		"expired_revoked_tokens": stats.expired_revoked_tokens
	})))
}

// Admin endpoint to force token cleanup
pub async fn force_cleanup(req: HttpRequest, state: web::Data<AppState>) -> Result<HttpResponse> {
	// Check if user is a manager
	let is_manager = req
		.extensions()
		.get::<stock_domain::entities::user::UserRole>()
		.is_some_and(|role| *role == stock_domain::entities::user::UserRole::Manager);

	if !is_manager {
		return Ok(HttpResponse::Forbidden().json(serde_json::json!({
			"error": "Manager role required"
		})));
	}

	let stats_before = state.blacklist_service.get_detailed_stats();
	state.blacklist_service.cleanup_expired_tokens();
	let stats_after = state.blacklist_service.get_stats();

	let active_cleaned = stats_before.basic.total_active_tokens - stats_after.total_active_tokens;
	let revoked_cleaned = stats_before.basic.revoked_tokens_count - stats_after.revoked_tokens_count;

	info!(
		"Manual token cleanup completed: {} active tokens cleaned, {} revoked tokens cleaned",
		active_cleaned, revoked_cleaned
	);

	Ok(HttpResponse::Ok().json(serde_json::json!({
		"message": "Cleanup completed",
		"tokens_cleaned": {
			"active": active_cleaned,
			"revoked": revoked_cleaned,
			"total": active_cleaned + revoked_cleaned
		},
		"stats_after": {
			"active_users": stats_after.active_users_count,
			"total_active_tokens": stats_after.total_active_tokens,
			"revoked_tokens": stats_after.revoked_tokens_count
		}
	})))
}

// Admin endpoint to revoke all tokens for a specific user
pub async fn revoke_user_tokens(
	req: HttpRequest,
	path: web::Path<uuid::Uuid>,
	state: web::Data<AppState>,
) -> Result<HttpResponse> {
	// Check if user is a manager
	let is_manager = req
		.extensions()
		.get::<stock_domain::entities::user::UserRole>()
		.is_some_and(|role| *role == stock_domain::entities::user::UserRole::Manager);

	if !is_manager {
		return Ok(HttpResponse::Forbidden().json(serde_json::json!({
			"error": "Manager role required"
		})));
	}

	let user_id = path.into_inner();
	let revoked_count = state.blacklist_service.revoke_user_tokens(user_id);

	info!("Admin revoked {} tokens for user {}", revoked_count, user_id);

	Ok(HttpResponse::Ok().json(serde_json::json!({
		"message": format!("Revoked {} tokens for user {}", revoked_count, user_id),
		"user_id": user_id,
		"tokens_revoked": revoked_count
	})))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::services::token_blacklist::TokenBlacklistService;
	use actix_web::test;
	use std::sync::Arc;
	use stock_application::services::auth_service::{AuthService, RegisterUserDto};
	use stock_domain::entities::user::UserRole;
	use stock_infrastructure::db::establish_connection_pool;
	use stock_infrastructure::repositories::user_repository::DieselUserRepository;

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
	async fn test_login_form() {
		let app_state = create_test_app_state();
		let response = login_form(app_state).await.unwrap();
		assert_eq!(response.status(), actix_web::http::StatusCode::OK);
	}

	#[actix_web::test]
	async fn test_register_form() {
		let app_state = create_test_app_state();
		let response = register_form(app_state).await.unwrap();
		assert_eq!(response.status(), actix_web::http::StatusCode::OK);
	}

	#[actix_web::test]
	async fn test_logout_without_token() {
		let app_state = create_test_app_state();
		let req = test::TestRequest::get().to_http_request();
		let response = logout(req, app_state).await.unwrap();

		assert_eq!(response.status(), actix_web::http::StatusCode::FOUND);
		assert_eq!(response.headers().get("Location").unwrap(), "/auth/login");
	}

	#[actix_web::test]
	async fn test_successful_login_logout_flow() {
		let app_state = create_test_app_state();

		// First register a user
		let register_dto = RegisterUserDto {
			username: "testuser".to_string(),
			password: "testpass".to_string(),
			role: UserRole::Manager,
		};

		let _user = app_state.auth_service.register(register_dto).await.unwrap();

		// Then login
		let login_form = web::Form(LoginDto {
			username: "testuser".to_string(),
			password: "testpass".to_string(),
		});

		let login_response = login(app_state.clone(), login_form).await.unwrap();
		assert_eq!(login_response.status(), actix_web::http::StatusCode::FOUND);

		// Extract token from response cookies
		let mut auth_token = None;
		for (name, value) in login_response.headers() {
			if name.as_str().to_lowercase() == "set-cookie" {
				let cookie_str = value.to_str().unwrap();
				if let Some(token_start) = cookie_str.find("auth_token=") {
					let token_start = token_start + "auth_token=".len();
					let token_end = cookie_str[token_start..]
						.find(';')
						.unwrap_or(cookie_str.len() - token_start);
					auth_token = Some(cookie_str[token_start..token_start + token_end].to_string());
					break;
				}
			}
		}

		assert!(auth_token.is_some(), "Auth token should be present in login response");

		// Test logout with the token
		let req = test::TestRequest::get()
			.cookie(actix_web::cookie::Cookie::new("auth_token", auth_token.unwrap()))
			.to_http_request();

		let logout_response = logout(req, app_state).await.unwrap();
		assert_eq!(logout_response.status(), actix_web::http::StatusCode::FOUND);
		assert_eq!(logout_response.headers().get("Location").unwrap(), "/auth/login");
	}

	#[actix_web::test]
	async fn test_token_stats_requires_manager() {
		let app_state = create_test_app_state();

		// Test without manager role
		let req = test::TestRequest::get().to_http_request();
		let response = token_stats(req, app_state.clone()).await.unwrap();
		assert_eq!(response.status(), actix_web::http::StatusCode::FORBIDDEN);

		// Test with manager role
		let req = test::TestRequest::get().to_http_request();
		req.extensions_mut().insert(UserRole::Manager);
		let response = token_stats(req, app_state).await.unwrap();
		assert_eq!(response.status(), actix_web::http::StatusCode::OK);
	}
}

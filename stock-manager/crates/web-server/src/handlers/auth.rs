use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use chrono::{DateTime, Utc};
use stock_application::services::auth_service::Credentials;

use crate::AppState;
use crate::dtos::auth::{LoginDto, LoginTemplate, RegisterDto, RegisterTemplate};
use crate::services::token_blacklist::TokenInfo;

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
		Ok(_) => Ok(HttpResponse::Found()
			.append_header(("Location", "/auth/login"))
			.finish()),
		Err(e) => Ok(HttpResponse::BadRequest().body(format!("Registration failed: {e}"))),
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
			let token_info = TokenInfo {
				jti: token.jti.clone(),
				user_id: token.user_id,
				expires_at: DateTime::from_timestamp(token.expires_at, 0).unwrap_or_else(Utc::now),
			};
			state.blacklist_service.register_token(token_info);

			// Set JWT token as a cookie
			Ok(HttpResponse::Found()
				.cookie(
					actix_web::cookie::Cookie::build("auth_token", token.token)
						.http_only(true)
						.same_site(actix_web::cookie::SameSite::Strict)
						.path("/")
						.finish(),
				)
				// Add username cookie for UI display
				.cookie(
					actix_web::cookie::Cookie::build("username", form.username.clone())
						.path("/")
						.finish(),
				)
				.append_header(("Location", "/"))
				.finish())
		},
		Err(e) => Ok(HttpResponse::BadRequest().body(format!("Login failed: {e}"))),
	}
}

pub async fn logout() -> Result<HttpResponse> {
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

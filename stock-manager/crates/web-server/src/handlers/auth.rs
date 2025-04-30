use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use stock_application::services::auth_service::Credentials;

use crate::AppState;
use crate::dtos::auth::{LoginDto, LoginTemplate, RegisterDto, RegisterTemplate};

pub async fn login_form() -> Result<HttpResponse> {
	let template = LoginTemplate {};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn register_form() -> Result<HttpResponse> {
	let template = RegisterTemplate {};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn register(state: web::Data<AppState>, form: web::Form<RegisterDto>) -> Result<HttpResponse> {
	let auth_service = &state.auth_service;

	let dto = form.0.into();

	match auth_service.register(dto).await {
		Ok(_) => Ok(HttpResponse::Found()
			.append_header(("Location", "/auth/login"))
			.finish()),
		Err(e) => Ok(HttpResponse::BadRequest().body(format!("Registration failed: {}", e))),
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
			// Set JWT token as a cookie
			Ok(HttpResponse::Found()
				.cookie(
					actix_web::cookie::Cookie::build("auth_token", token.token.clone())
						.http_only(true)
						.same_site(actix_web::cookie::SameSite::Strict)
						.path("/")
						.finish(),
				)
				.append_header(("Location", "/"))
				.finish())
		},
		Err(e) => Ok(HttpResponse::BadRequest().body(format!("Login failed: {}", e))),
	}
}

pub async fn logout() -> Result<HttpResponse> {
	// Clear the auth cookie
	Ok(HttpResponse::Found()
		.cookie(
			actix_web::cookie::Cookie::build("auth_token", "")
				.http_only(true)
				.same_site(actix_web::cookie::SameSite::Strict)
				.path("/")
				.max_age(actix_web::cookie::time::Duration::seconds(-1))
				.finish(),
		)
		.append_header(("Location", "/auth/login"))
		.finish())
}

use askama::Template;
use serde::{Deserialize, Serialize};
use stock_domain::entities::user::UserRole;

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDto {
	pub username: String,
	pub password: String,
	pub role: String,
}

impl From<RegisterDto> for stock_application::services::auth_service::RegisterUserDto {
	fn from(dto: RegisterDto) -> Self {
		Self {
			username: dto.username,
			password: dto.password,
			role: match dto.role.to_uppercase().as_str() {
				"MANAGER" => UserRole::Manager,
				_ => UserRole::Seller,
			},
		}
	}
}

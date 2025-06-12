use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use stock_domain::entities::user::{User, UserRole};
use stock_domain::repositories::user_repository::UserRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserDto {
	pub username: String,
	pub password: String,
	pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenDto {
	pub token: String,
	pub jti: String,
	pub user_id: Uuid,
	pub role: UserRole,
	pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	pub sub: String,      // Subject (User ID)
	pub username: String, // Username
	pub role: String,     // Role
	pub exp: i64,         // Expiration time
	pub iat: i64,         // Issued at
	pub jti: String,      // JWT ID
}

pub struct AuthService {
	pub user_repository: Arc<dyn UserRepository>,
	jwt_secret: String,
}

impl AuthService {
	pub fn new(user_repository: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
		Self {
			user_repository,
			jwt_secret,
		}
	}

	pub async fn register(&self, dto: RegisterUserDto) -> anyhow::Result<User> {
		// Check if username already exists
		if (self.user_repository.find_by_username(&dto.username).await?).is_some() {
			return Err(anyhow::anyhow!("Username already exists"));
		}

		// Hash the password using repository method
		let hashed_password = self.user_repository.hash_password(&dto.password).await?;

		let now = Utc::now();
		let user = User {
			id: Uuid::new_v4(),
			username: dto.username,
			password_hash: hashed_password,
			role: dto.role,
			created_at: now,
			updated_at: now,
		};

		self.user_repository.create(user).await
	}

	pub async fn login(&self, credentials: Credentials) -> anyhow::Result<AuthTokenDto> {
		let user = self
			.user_repository
			.find_by_username(&credentials.username)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

		// Use repository method to verify password
		let is_valid = self
			.user_repository
			.verify_password(&credentials.password, &user.password_hash)
			.await?;

		if !is_valid {
			return Err(anyhow::anyhow!("Invalid credentials"));
		}

		// Generate JWT token with JTI
		let (token, jti) = self.generate_token(&user)?;

		// Get expiration time (e.g., 24 hours from now)
		let now = Utc::now();
		let expiration = now + Duration::hours(24);

		Ok(AuthTokenDto {
			token,
			jti,
			user_id: user.id,
			role: user.role,
			expires_at: expiration.timestamp(),
		})
	}

	pub fn verify_token(&self, token: &str) -> anyhow::Result<TokenData<Claims>> {
		let token_data = decode::<Claims>(
			token,
			&DecodingKey::from_secret(self.jwt_secret.as_bytes()),
			&Validation::default(),
		)?;

		Ok(token_data)
	}

	fn generate_token(&self, user: &User) -> anyhow::Result<(String, String)> {
		let now = Utc::now();
		let expires_at = now + Duration::hours(24);
		let jti = Uuid::new_v4().to_string();

		let claims = Claims {
			sub: user.id.to_string(),
			username: user.username.clone(),
			role: user.role.to_string(),
			exp: expires_at.timestamp(),
			iat: now.timestamp(),
			jti: jti.clone(),
		};

		let token = encode(
			&Header::default(),
			&claims,
			&EncodingKey::from_secret(self.jwt_secret.as_bytes()),
		)?;

		Ok((token, jti))
	}
}

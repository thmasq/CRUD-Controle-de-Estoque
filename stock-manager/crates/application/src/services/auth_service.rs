use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info, warn};
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
		debug!("Attempting to register user: {}", dto.username);

		// Check if username already exists
		if (self.user_repository.find_by_username(&dto.username).await?).is_some() {
			warn!("Registration failed: username '{}' already exists", dto.username);
			return Err(anyhow::anyhow!("Username already exists"));
		}

		// Hash the password using repository method
		let hashed_password = self.user_repository.hash_password(&dto.password).await.map_err(|e| {
			error!("Password hashing failed for user '{}': {}", dto.username, e);
			anyhow::anyhow!("Password hashing failed: {}", e)
		})?;

		let now = Utc::now();
		let user = User {
			id: Uuid::new_v4(),
			username: dto.username.clone(),
			password_hash: hashed_password,
			role: dto.role,
			created_at: now,
			updated_at: now,
		};

		let created_user = self.user_repository.create(user).await.map_err(|e| {
			error!("User creation failed for '{}': {}", dto.username, e);
			anyhow::anyhow!("User creation failed: {}", e)
		})?;

		info!(
			"User '{}' registered successfully with ID: {}",
			created_user.username, created_user.id
		);
		Ok(created_user)
	}

	pub async fn login(&self, credentials: Credentials) -> anyhow::Result<AuthTokenDto> {
		debug!("Login attempt for user: {}", credentials.username);

		// Find user by username
		let user = self
			.user_repository
			.find_by_username(&credentials.username)
			.await
			.map_err(|e| {
				error!(
					"Database error during user lookup for '{}': {}",
					credentials.username, e
				);
				anyhow::anyhow!("Database error during authentication")
			})?
			.ok_or_else(|| {
				warn!("Login failed: user '{}' not found", credentials.username);
				anyhow::anyhow!("Invalid credentials")
			})?;

		debug!("User '{}' found in database, verifying password", credentials.username);

		// Use repository method to verify password
		let is_valid = self
			.user_repository
			.verify_password(&credentials.password, &user.password_hash)
			.await
			.map_err(|e| {
				error!("Password verification error for user '{}': {}", credentials.username, e);
				anyhow::anyhow!("Password verification failed")
			})?;

		if !is_valid {
			warn!("Login failed: invalid password for user '{}'", credentials.username);
			return Err(anyhow::anyhow!("Invalid credentials"));
		}

		debug!("Password verified successfully for user '{}'", credentials.username);

		// Generate JWT token with JTI
		let (token, jti) = self.generate_token(&user).map_err(|e| {
			error!("Token generation failed for user '{}': {}", credentials.username, e);
			anyhow::anyhow!("Token generation failed")
		})?;

		// Get expiration time (1 hour from now)
		let now = Utc::now();
		let expiration = now + Duration::hours(1);

		info!(
			"Login successful for user '{}' (ID: {}), JTI: {}",
			user.username, user.id, jti
		);

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

	pub async fn refresh_token(&self, user_id: Uuid) -> anyhow::Result<AuthTokenDto> {
		debug!("Refreshing token for user ID: {}", user_id);

		let user = self
			.user_repository
			.find_by_id(user_id)
			.await?
			.ok_or_else(|| anyhow::anyhow!("User not found"))?;

		let (token, jti) = self.generate_token(&user)?;

		let now = Utc::now();
		let expiration = now + Duration::hours(1);

		debug!("Token refreshed for user '{}', new JTI: {}", user.username, jti);

		Ok(AuthTokenDto {
			token,
			jti,
			user_id: user.id,
			role: user.role,
			expires_at: expiration.timestamp(),
		})
	}

	pub fn generate_token_from_claims(&self, claims: &Claims) -> anyhow::Result<(String, String)> {
		let now = Utc::now();
		let expires_at = now + Duration::hours(1);
		let jti = Uuid::new_v4().to_string();

		debug!(
			"Generating new token from existing claims for user '{}', new JTI: {}",
			claims.username, jti
		);

		let new_claims = Claims {
			sub: claims.sub.clone(),
			username: claims.username.clone(),
			role: claims.role.clone(),
			exp: expires_at.timestamp(),
			iat: now.timestamp(),
			jti: jti.clone(),
		};

		let token = encode(
			&Header::default(),
			&new_claims,
			&EncodingKey::from_secret(self.jwt_secret.as_bytes()),
		)?;

		Ok((token, jti))
	}

	fn generate_token(&self, user: &User) -> anyhow::Result<(String, String)> {
		let now = Utc::now();
		let expires_at = now + Duration::hours(1);
		let jti = Uuid::new_v4().to_string();

		debug!(
			"Generating token for user '{}' (ID: {}), JTI: {}",
			user.username, user.id, jti
		);

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

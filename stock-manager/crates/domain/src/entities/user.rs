use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
	Seller,
	Manager,
}

impl std::fmt::Display for UserRole {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Seller => write!(f, "SELLER"),
			Self::Manager => write!(f, "MANAGER"),
		}
	}
}

impl From<&str> for UserRole {
	fn from(role: &str) -> Self {
		match role.to_uppercase().as_str() {
			"MANAGER" => UserRole::Manager,
			_ => UserRole::Seller,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	#[serde(skip_serializing)]
	pub password_hash: String,
	pub role: UserRole,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

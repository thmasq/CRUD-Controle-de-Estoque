use chrono::{DateTime, Utc};
use diesel::prelude::*;
use stock_domain::entities::user::{User, UserRole};
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
	pub id: Uuid,
	pub username: String,
	pub password_hash: String,
	pub role: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUserModel {
	pub id: Uuid,
	pub username: String,
	pub password_hash: String,
	pub role: String,
}

impl From<UserModel> for User {
	fn from(model: UserModel) -> Self {
		let role = match model.role.as_str() {
			"MANAGER" => UserRole::Manager,
			_ => UserRole::Seller,
		};

		Self {
			id: model.id,
			username: model.username,
			password_hash: model.password_hash,
			role,
			created_at: model.created_at,
			updated_at: model.updated_at,
		}
	}
}

impl From<User> for NewUserModel {
	fn from(entity: User) -> Self {
		Self {
			id: entity.id,
			username: entity.username,
			password_hash: entity.password_hash,
			role: entity.role.to_string(),
		}
	}
}

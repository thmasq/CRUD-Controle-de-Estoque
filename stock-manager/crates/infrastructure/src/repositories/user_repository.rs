use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Text};
use std::sync::Arc;
use stock_domain::entities::user::User;
use stock_domain::repositories::user_repository::UserRepository;
use uuid::Uuid;

use crate::db::PgPool;
use crate::models::user::{NewUserModel, UserModel};
use crate::schema::users;

pub struct DieselUserRepository {
	pool: Arc<PgPool>,
}

impl DieselUserRepository {
	#[must_use]
	pub const fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl UserRepository for DieselUserRepository {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
		let conn = &mut self.pool.get()?;

		let result = users::table
			.find(id)
			.select(UserModel::as_select())
			.first(conn)
			.optional()?;

		Ok(result.map(Into::into))
	}

	async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
		let conn = &mut self.pool.get()?;

		let result = users::table
			.filter(users::username.eq(username))
			.select(UserModel::as_select())
			.first(conn)
			.optional()?;

		Ok(result.map(Into::into))
	}

	async fn find_all(&self) -> anyhow::Result<Vec<User>> {
		let conn = &mut self.pool.get()?;
		let result = users::table.select(UserModel::as_select()).load(conn)?;
		Ok(result.into_iter().map(Into::into).collect())
	}

	async fn create(&self, user: User) -> anyhow::Result<User> {
		let conn = &mut self.pool.get()?;

		let new_user = NewUserModel::from(user);

		diesel::insert_into(users::table).values(&new_user).execute(conn)?;

		let inserted_user = users::table
			.find(new_user.id)
			.select(UserModel::as_select())
			.first(conn)?;

		Ok(inserted_user.into())
	}

	async fn update(&self, user: User) -> anyhow::Result<User> {
		let conn = &mut self.pool.get()?;

		let updated_rows = diesel::update(users::table.find(user.id))
			.set((
				users::username.eq(user.username.clone()),
				users::password_hash.eq(user.password_hash.clone()),
				users::role.eq(user.role.to_string()),
				users::updated_at.eq(Utc::now()),
			))
			.execute(conn)?;

		if updated_rows == 0 {
			return Err(anyhow::anyhow!("User not found"));
		}

		let updated_user = users::table.find(user.id).select(UserModel::as_select()).first(conn)?;

		Ok(updated_user.into())
	}

	async fn delete(&self, id: Uuid) -> anyhow::Result<bool> {
		let conn = &mut self.pool.get()?;

		let deleted_rows = diesel::delete(users::table.find(id)).execute(conn)?;

		Ok(deleted_rows > 0)
	}

	async fn hash_password(&self, password: &str) -> anyhow::Result<String> {
		let conn = &mut self.pool.get()?;

		let result = diesel::sql_query("SELECT hash_password($1) as result")
			.bind::<Text, _>(password)
			.load::<HashResult>(conn)?
			.pop()
			.map(|r| r.result)
			.ok_or_else(|| anyhow::anyhow!("Failed to hash password"))?;

		Ok(result)
	}

	async fn verify_password(&self, password: &str, password_hash: &str) -> anyhow::Result<bool> {
		let conn = &mut self.pool.get()?;

		let result = diesel::sql_query("SELECT verify_password($1, $2) as result")
			.bind::<Text, _>(password)
			.bind::<Text, _>(password_hash)
			.load::<VerifyResult>(conn)?
			.pop()
			.is_some_and(|r| r.result);

		Ok(result)
	}
}

#[derive(QueryableByName, Debug)]
struct HashResult {
	#[diesel(sql_type = Text)]
	result: String,
}

#[derive(QueryableByName, Debug)]
struct VerifyResult {
	#[diesel(sql_type = Bool)]
	result: bool,
}

use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
	async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
	async fn find_all(&self) -> anyhow::Result<Vec<User>>;
	async fn create(&self, user: User) -> anyhow::Result<User>;
	async fn update(&self, user: User) -> anyhow::Result<User>;
	async fn delete(&self, id: Uuid) -> anyhow::Result<bool>;
	async fn hash_password(&self, password: &str) -> anyhow::Result<String>;
	async fn verify_password(&self, password: &str, password_hash: &str) -> anyhow::Result<bool>;
}

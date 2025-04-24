use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::category::Category;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>>;
	async fn find_all(&self) -> anyhow::Result<Vec<Category>>;
	async fn create(&self, category: Category) -> anyhow::Result<Category>;
	async fn update(&self, category: Category) -> anyhow::Result<Category>;
	async fn delete(&self, id: Uuid) -> anyhow::Result<bool>;
}

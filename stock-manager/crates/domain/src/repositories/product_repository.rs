use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::product::Product;

#[async_trait]
pub trait ProductRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Product>>;
	async fn find_by_sku(&self, sku: &str) -> anyhow::Result<Option<Product>>;
	async fn find_all(&self) -> anyhow::Result<Vec<Product>>;
	async fn find_by_category(&self, category_id: Uuid) -> anyhow::Result<Vec<Product>>;
	async fn create(&self, product: Product) -> anyhow::Result<Product>;
	async fn update(&self, product: Product) -> anyhow::Result<Product>;
	async fn delete(&self, id: Uuid) -> anyhow::Result<bool>;
}

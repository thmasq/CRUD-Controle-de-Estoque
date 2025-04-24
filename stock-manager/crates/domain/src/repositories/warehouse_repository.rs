use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::warehouse::Warehouse;

#[async_trait]
pub trait WarehouseRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Warehouse>>;
	async fn find_all(&self) -> anyhow::Result<Vec<Warehouse>>;
	async fn create(&self, warehouse: Warehouse) -> anyhow::Result<Warehouse>;
	async fn update(&self, warehouse: Warehouse) -> anyhow::Result<Warehouse>;
	async fn delete(&self, id: Uuid) -> anyhow::Result<bool>;
}

use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::stock_item::StockItem;

#[async_trait]
pub trait StockItemRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<StockItem>>;
	async fn find_all(&self) -> anyhow::Result<Vec<StockItem>>;
	async fn find_by_product(&self, product_id: Uuid) -> anyhow::Result<Vec<StockItem>>;
	async fn find_by_warehouse(&self, warehouse_id: Uuid) -> anyhow::Result<Vec<StockItem>>;
	async fn find_by_product_and_warehouse(
		&self,
		product_id: Uuid,
		warehouse_id: Uuid,
	) -> anyhow::Result<Option<StockItem>>;
	async fn create(&self, stock_item: StockItem) -> anyhow::Result<StockItem>;
	async fn update(&self, stock_item: StockItem) -> anyhow::Result<StockItem>;
	async fn delete(&self, id: Uuid) -> anyhow::Result<bool>;
}

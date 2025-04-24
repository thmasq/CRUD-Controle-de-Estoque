use std::sync::Arc;

use chrono::Utc;
use rust_decimal::Decimal;
use stock_domain::entities::stock_item::StockItem;
use stock_domain::repositories::stock_item_repository::StockItemRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StockItemCreateDto {
	pub product_id: Uuid,
	pub warehouse_id: Uuid,
	pub quantity: i32,
	pub unit_cost: Decimal,
}

#[derive(Debug, Clone)]
pub struct StockItemUpdateDto {
	pub id: Uuid,
	pub quantity: i32,
	pub unit_cost: Decimal,
}

pub struct StockItemService {
	repository: Arc<dyn StockItemRepository>,
}

impl StockItemService {
	pub fn new(repository: Arc<dyn StockItemRepository>) -> Self {
		Self { repository }
	}

	pub async fn get_stock_item(&self, id: Uuid) -> anyhow::Result<Option<StockItem>> {
		self.repository.find_by_id(id).await
	}

	pub async fn get_all_stock_items(&self) -> anyhow::Result<Vec<StockItem>> {
		self.repository.find_all().await
	}

	pub async fn get_stock_items_by_product(&self, product_id: Uuid) -> anyhow::Result<Vec<StockItem>> {
		self.repository.find_by_product(product_id).await
	}

	pub async fn get_stock_items_by_warehouse(&self, warehouse_id: Uuid) -> anyhow::Result<Vec<StockItem>> {
		self.repository.find_by_warehouse(warehouse_id).await
	}

	pub async fn create_stock_item(&self, dto: StockItemCreateDto) -> anyhow::Result<StockItem> {
		// Check if a stock item for this product and warehouse already exists
		if let Some(_) = self
			.repository
			.find_by_product_and_warehouse(dto.product_id, dto.warehouse_id)
			.await?
		{
			return Err(anyhow::anyhow!(
				"Stock item for this product and warehouse already exists"
			));
		}

		let now = Utc::now();
		let stock_item = StockItem {
			id: Uuid::new_v4(),
			product_id: dto.product_id,
			warehouse_id: dto.warehouse_id,
			quantity: dto.quantity,
			unit_cost: dto.unit_cost,
			last_restocked: now,
			created_at: now,
			updated_at: now,
		};

		self.repository.create(stock_item).await
	}

	pub async fn update_stock_item(&self, dto: StockItemUpdateDto) -> anyhow::Result<StockItem> {
		let existing = self
			.repository
			.find_by_id(dto.id)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Stock item not found"))?;

		let stock_item = StockItem {
			id: existing.id,
			product_id: existing.product_id,
			warehouse_id: existing.warehouse_id,
			quantity: dto.quantity,
			unit_cost: dto.unit_cost,
			last_restocked: Utc::now(), // Update last_restocked when updating stock
			created_at: existing.created_at,
			updated_at: Utc::now(),
		};

		self.repository.update(stock_item).await
	}

	pub async fn delete_stock_item(&self, id: Uuid) -> anyhow::Result<bool> {
		self.repository.delete(id).await
	}
}

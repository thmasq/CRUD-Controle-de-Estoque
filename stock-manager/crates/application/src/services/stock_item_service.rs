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

	pub async fn get_all_stock_items_with_inactive(&self) -> anyhow::Result<Vec<StockItem>> {
		self.repository.find_all_with_inactive().await
	}

	pub async fn get_stock_items_by_product(&self, product_id: Uuid) -> anyhow::Result<Vec<StockItem>> {
		self.repository.find_by_product(product_id).await
	}

	pub async fn get_stock_items_by_warehouse(&self, warehouse_id: Uuid) -> anyhow::Result<Vec<StockItem>> {
		self.repository.find_by_warehouse(warehouse_id).await
	}

	pub async fn create_stock_item(&self, dto: StockItemCreateDto) -> anyhow::Result<StockItem> {
		// Check if a stock item (including inactive) for this product and warehouse already exists
		let existing_item_result = self
			.repository
			.find_by_product_and_warehouse_with_inactive(dto.product_id, dto.warehouse_id)
			.await;

		if let Ok(Some(existing_item)) = existing_item_result {
			if !existing_item.is_active {
				// Item exists but is marked as deleted - reactivate it with new values
				let now = Utc::now();
				let reactivated_item = StockItem {
					id: existing_item.id,
					product_id: existing_item.product_id,
					warehouse_id: existing_item.warehouse_id,
					quantity: dto.quantity,
					unit_cost: dto.unit_cost,
					last_restocked: now,
					is_active: true, // Explicitly set to true for reactivation
					created_at: existing_item.created_at,
					updated_at: now,
				};

				// Update the item to reactivate it
				return self.repository.update(reactivated_item).await;
			} else {
				// Active item already exists
				return Err(anyhow::anyhow!(
					"Stock item for this product and warehouse already exists"
				));
			}
		}

		let now = Utc::now();
		let stock_item = StockItem {
			id: Uuid::new_v4(),
			product_id: dto.product_id,
			warehouse_id: dto.warehouse_id,
			quantity: dto.quantity,
			unit_cost: dto.unit_cost,
			last_restocked: now,
			is_active: true,
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
			last_restocked: Utc::now(),
			is_active: existing.is_active,
			created_at: existing.created_at,
			updated_at: Utc::now(),
		};

		self.repository.update(stock_item).await
	}

	pub async fn delete_stock_item(&self, id: Uuid) -> anyhow::Result<bool> {
		self.repository.delete(id).await
	}
}

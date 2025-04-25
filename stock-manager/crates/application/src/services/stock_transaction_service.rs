use std::sync::Arc;

use stock_domain::entities::stock_item::StockItem;
use stock_domain::entities::stock_transaction::{StockTransaction, TransactionType};
use stock_domain::repositories::stock_item_repository::StockItemRepository;
use stock_domain::repositories::stock_transaction_repository::StockTransactionRepository;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StockTransactionCreateDto {
	pub stock_item_id: Uuid,
	pub quantity: i32,
	pub transaction_type: TransactionType,
	pub reference_number: Option<String>,
	pub notes: Option<String>,
	pub created_by: String,
}

pub struct StockTransactionService {
	transaction_repository: Arc<dyn StockTransactionRepository>,
	stock_item_repository: Arc<dyn StockItemRepository>,
}

impl StockTransactionService {
	pub fn new(
		transaction_repository: Arc<dyn StockTransactionRepository>,
		stock_item_repository: Arc<dyn StockItemRepository>,
	) -> Self {
		Self {
			transaction_repository,
			stock_item_repository,
		}
	}

	pub async fn get_transaction(&self, id: Uuid) -> anyhow::Result<Option<StockTransaction>> {
		self.transaction_repository.find_by_id(id).await
	}

	pub async fn get_all_transactions(&self) -> anyhow::Result<Vec<StockTransaction>> {
		self.transaction_repository.find_all().await
	}

	pub async fn get_transactions_by_stock_item(&self, stock_item_id: Uuid) -> anyhow::Result<Vec<StockTransaction>> {
		self.transaction_repository.find_by_stock_item(stock_item_id).await
	}

	pub async fn create_transaction(&self, dto: StockTransactionCreateDto) -> anyhow::Result<StockTransaction> {
		// Find the stock item (including inactive ones)
		let stock_item = self
			.stock_item_repository
			.find_by_id_with_inactive(dto.stock_item_id)
			.await?
			.ok_or_else(|| anyhow::anyhow!("Stock item not found"))?;

		if !stock_item.is_active {
			return Err(anyhow::anyhow!("Cannot create transaction for inactive stock item"));
		}

		// Calculate new quantity based on transaction type
		let new_quantity = match dto.transaction_type {
			TransactionType::In => stock_item.quantity + dto.quantity,
			TransactionType::Out => {
				if stock_item.quantity < dto.quantity {
					return Err(anyhow::anyhow!("Insufficient stock"));
				}
				stock_item.quantity - dto.quantity
			},
			TransactionType::Adjustment => dto.quantity,
		};

		// Update stock item
		let updated_stock_item = StockItem {
			id: stock_item.id,
			product_id: stock_item.product_id,
			warehouse_id: stock_item.warehouse_id,
			quantity: new_quantity,
			unit_cost: stock_item.unit_cost,
			last_restocked: stock_item.last_restocked,
			created_at: stock_item.created_at,
			updated_at: chrono::Utc::now(),
			is_active: stock_item.is_active,
		};
		self.stock_item_repository.update(updated_stock_item).await?;

		// Create transaction record
		let transaction = StockTransaction {
			id: Uuid::new_v4(),
			stock_item_id: dto.stock_item_id,
			quantity: dto.quantity,
			transaction_type: dto.transaction_type,
			reference_number: dto.reference_number,
			notes: dto.notes,
			created_at: chrono::Utc::now(),
			created_by: dto.created_by,
		};

		self.transaction_repository.create(transaction).await
	}
}

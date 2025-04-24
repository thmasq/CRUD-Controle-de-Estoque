use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::stock_transaction::{StockTransaction, TransactionType};

#[async_trait]
pub trait StockTransactionRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<StockTransaction>>;
	async fn find_all(&self) -> anyhow::Result<Vec<StockTransaction>>;
	async fn find_by_stock_item(&self, stock_item_id: Uuid) -> anyhow::Result<Vec<StockTransaction>>;
	async fn find_by_type(&self, transaction_type: TransactionType) -> anyhow::Result<Vec<StockTransaction>>;
	async fn create(&self, transaction: StockTransaction) -> anyhow::Result<StockTransaction>;
	// Transactions should not be updated or deleted for audit purposes. Talk to the DBA if necessary
	// lol
}

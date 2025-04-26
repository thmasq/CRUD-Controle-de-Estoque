use std::sync::Arc;

use async_trait::async_trait;
use diesel::prelude::*;
use stock_domain::entities::stock_transaction::{StockTransaction, TransactionType};
use stock_domain::repositories::stock_transaction_repository::StockTransactionRepository;
use uuid::Uuid;

use crate::db::PgPool;
use crate::models::stock_transaction::{NewStockTransactionModel, StockTransactionModel};
use crate::schema::stock_transactions;

pub struct DieselStockTransactionRepository {
	pool: Arc<PgPool>,
}

impl DieselStockTransactionRepository {
	#[must_use]
	pub const fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl StockTransactionRepository for DieselStockTransactionRepository {
	async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<StockTransaction>> {
		let conn = &mut self.pool.get()?;

		let result = stock_transactions::table
			.find(id)
			.select(StockTransactionModel::as_select())
			.first(conn)
			.optional()?;

		Ok(result.map(Into::into))
	}

	async fn find_all(&self) -> anyhow::Result<Vec<StockTransaction>> {
		let conn = &mut self.pool.get()?;

		let result = stock_transactions::table
			.select(StockTransactionModel::as_select())
			.load(conn)?;

		Ok(result.into_iter().map(Into::into).collect())
	}

	async fn find_by_stock_item(&self, stock_item_id: Uuid) -> anyhow::Result<Vec<StockTransaction>> {
		let conn = &mut self.pool.get()?;

		let result = stock_transactions::table
			.filter(stock_transactions::stock_item_id.eq(stock_item_id))
			.select(StockTransactionModel::as_select())
			.load(conn)?;

		Ok(result.into_iter().map(Into::into).collect())
	}

	async fn find_by_type(&self, transaction_type: TransactionType) -> anyhow::Result<Vec<StockTransaction>> {
		let conn = &mut self.pool.get()?;

		let type_str = transaction_type.to_string();

		let result = stock_transactions::table
			.filter(stock_transactions::transaction_type.eq(type_str))
			.select(StockTransactionModel::as_select())
			.load(conn)?;

		Ok(result.into_iter().map(Into::into).collect())
	}

	async fn create(&self, transaction: StockTransaction) -> anyhow::Result<StockTransaction> {
		let conn = &mut self.pool.get()?;

		let new_transaction = NewStockTransactionModel::from(transaction);

		diesel::insert_into(stock_transactions::table)
			.values(&new_transaction)
			.execute(conn)?;

		let inserted_transaction = stock_transactions::table
			.find(new_transaction.id)
			.select(StockTransactionModel::as_select())
			.first(conn)?;

		Ok(inserted_transaction.into())
	}
}

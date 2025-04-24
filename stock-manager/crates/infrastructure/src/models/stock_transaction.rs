use chrono::{DateTime, Utc};
use diesel::prelude::*;
use stock_domain::entities::stock_transaction::{StockTransaction, TransactionType};
use uuid::Uuid;

use crate::schema::stock_transactions;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = stock_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StockTransactionModel {
	pub id: Uuid,
	pub stock_item_id: Uuid,
	pub quantity: i32,
	pub transaction_type: String,
	pub reference_number: Option<String>,
	pub notes: Option<String>,
	pub created_at: DateTime<Utc>,
	pub created_by: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = stock_transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStockTransactionModel {
	pub id: Uuid,
	pub stock_item_id: Uuid,
	pub quantity: i32,
	pub transaction_type: String,
	pub reference_number: Option<String>,
	pub notes: Option<String>,
	pub created_by: String,
}

impl From<StockTransactionModel> for StockTransaction {
	fn from(model: StockTransactionModel) -> Self {
		let transaction_type = match model.transaction_type.as_str() {
			"IN" => TransactionType::In,
			"OUT" => TransactionType::Out,
			"ADJUSTMENT" => TransactionType::Adjustment,
			_ => TransactionType::Adjustment, // Default fallback
		};

		Self {
			id: model.id,
			stock_item_id: model.stock_item_id,
			quantity: model.quantity,
			transaction_type,
			reference_number: model.reference_number,
			notes: model.notes,
			created_at: model.created_at,
			created_by: model.created_by,
		}
	}
}

impl From<StockTransaction> for NewStockTransactionModel {
	fn from(entity: StockTransaction) -> Self {
		Self {
			id: entity.id,
			stock_item_id: entity.stock_item_id,
			quantity: entity.quantity,
			transaction_type: entity.transaction_type.to_string(),
			reference_number: entity.reference_number,
			notes: entity.notes,
			created_by: entity.created_by,
		}
	}
}

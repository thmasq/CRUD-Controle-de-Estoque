use askama::Template;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "transactions/index.html")]
pub struct TransactionListTemplate {
	pub transactions: Vec<TransactionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDto {
	pub id: Uuid,
	pub stock_item_id: Uuid,
	pub product_name: String,
	pub warehouse_name: String,
	pub quantity: i32,
	pub transaction_type: String,
	pub reference_number: Option<String>,
	pub notes: Option<String>,
	pub created_at: DateTime<Utc>,
	pub created_by: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionCreateRequest {
	pub transaction_type: String,
	pub quantity: i32,
	pub reference_number: Option<String>,
	pub notes: Option<String>,
	pub created_by: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionFilterRequest {
	pub transaction_type: Option<String>,
	pub date: Option<String>,
}

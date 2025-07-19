use crate::filters;
use askama::Template;
use serde::{Deserialize, Serialize};

use super::stock_item::StockItemDto;
use super::stock_transaction::TransactionDto;

#[derive(Template)]
#[template(path = "index.html")]
pub struct DashboardTemplate {
	pub product_count: u64,
	pub category_count: u64,
	pub warehouse_count: u64,
	pub stock_item_count: u64,
	pub recent_transactions: Vec<TransactionDto>,
	pub low_stock_items: Vec<StockItemDto>,
	pub transaction_chart_json: String,
	pub warehouse_chart_json: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionChartData {
	pub labels: Vec<String>,
	pub datasets: Vec<TransactionDataset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDataset {
	pub label: String,
	pub data: Vec<u32>,
	pub border_color: String,
	pub background_color: String,
	pub tension: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WarehouseChartData {
	pub labels: Vec<String>,
	pub data: Vec<u32>,
	pub background_colors: Vec<String>,
}

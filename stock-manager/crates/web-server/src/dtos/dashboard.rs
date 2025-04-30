use crate::filters;
use askama::Template;

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
}

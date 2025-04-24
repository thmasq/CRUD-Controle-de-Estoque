use askama::Template;

use super::stock_item::StockItemDto;
use super::stock_transaction::TransactionDto;

#[derive(Template)]
#[template(path = "index.html")]
pub struct DashboardTemplate {
	pub product_count: i64,
	pub category_count: i64,
	pub warehouse_count: i64,
	pub stock_item_count: i64,
	pub recent_transactions: Vec<TransactionDto>,
	pub low_stock_items: Vec<StockItemDto>,
}

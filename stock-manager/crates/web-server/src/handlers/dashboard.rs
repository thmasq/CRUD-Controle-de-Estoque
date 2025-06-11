use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;

use crate::AppState;
use crate::dtos::dashboard::DashboardTemplate;

pub async fn index(state: web::Data<AppState>) -> Result<HttpResponse> {
	// Create services
	let category_service = state.category_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();
	let stock_item_service = state.stock_item_service.clone();
	let transaction_service = state.transaction_service.clone();

	// Get counts
	let categories = category_service.get_all_categories().await.unwrap_or_default();
	let products = product_service.get_all_products().await.unwrap_or_default();
	let warehouses = warehouse_service.get_all_warehouses().await.unwrap_or_default();
	let stock_items = stock_item_service.get_all_stock_items().await.unwrap_or_default();
	let transactions = transaction_service.get_all_transactions().await.unwrap_or_default();

	// Prepare low stock items
	let mut low_stock_items = Vec::new();
	let product_map: std::collections::HashMap<_, _> = products.iter().map(|p| (p.id, p.clone())).collect();
	let warehouse_map: std::collections::HashMap<_, _> = warehouses.iter().map(|w| (w.id, w.clone())).collect();

	for item in &stock_items {
		if item.quantity < 10 {
			// Low stock threshold
			if let (Some(product), Some(warehouse)) =
				(product_map.get(&item.product_id), warehouse_map.get(&item.warehouse_id))
			{
				low_stock_items.push(crate::dtos::stock_item::StockItemDto {
					id: item.id,
					product_id: item.product_id,
					product_name: product.name.clone(),
					warehouse_id: item.warehouse_id,
					warehouse_name: warehouse.name.clone(),
					quantity: item.quantity,
					unit_cost: item.unit_cost,
					last_restocked: item.last_restocked,
					is_active: item.is_active,
				});
			}
		}
	}

	// Prepare recent transactions
	let mut recent_transactions = Vec::new();
	for transaction in transactions.iter().take(5) {
		// Take most recent 5
		if let Some(stock_item) = stock_items.iter().find(|si| si.id == transaction.stock_item_id)
			&& let (Some(product), Some(warehouse)) = (
				product_map.get(&stock_item.product_id),
				warehouse_map.get(&stock_item.warehouse_id),
			) {
			recent_transactions.push(crate::dtos::stock_transaction::TransactionDto {
				id: transaction.id,
				stock_item_id: transaction.stock_item_id,
				product_name: product.name.clone(),
				warehouse_name: warehouse.name.clone(),
				quantity: transaction.quantity,
				transaction_type: transaction.transaction_type.to_string(),
				reference_number: transaction.reference_number.clone(),
				notes: transaction.notes.clone(),
				created_at: transaction.created_at,
				created_by: transaction.created_by.clone(),
			});
		}
	}

	// Create template
	let template = DashboardTemplate {
		product_count: products.len() as u64,
		category_count: categories.len() as u64,
		warehouse_count: warehouses.len() as u64,
		stock_item_count: stock_items.len() as u64,
		recent_transactions,
		low_stock_items,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

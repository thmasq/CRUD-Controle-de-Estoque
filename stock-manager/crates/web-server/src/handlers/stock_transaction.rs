use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use chrono::{NaiveDate, TimeZone, Utc};

use crate::AppState;
use crate::dtos::stock_transaction::{TransactionDto, TransactionFilterRequest, TransactionListTemplate};

pub async fn list_transactions(
	state: web::Data<AppState>,
	query: web::Query<TransactionFilterRequest>,
) -> Result<HttpResponse> {
	let transaction_service = state.transaction_service.clone();
	let stock_service = state.stock_item_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	// Get all transactions
	let mut transactions = transaction_service.get_all_transactions().await.unwrap_or_default();

	// Filter by transaction type if specified
	if let Some(ref transaction_type) = query.transaction_type {
		transactions = transactions
			.into_iter()
			.filter(|t| t.transaction_type.to_string() == *transaction_type)
			.collect();
	}

	// Filter by date if specified
	if let Some(ref date_str) = query.date {
		if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
			let start_of_day = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap());
			let end_of_day = Utc.from_utc_datetime(&date.and_hms_opt(23, 59, 59).unwrap());

			transactions = transactions
				.into_iter()
				.filter(|t| t.created_at >= start_of_day && t.created_at <= end_of_day)
				.collect();
		}
	}

	// Get all stock items
	let stock_items = stock_service
		.get_all_stock_items_with_inactive()
		.await
		.unwrap_or_default();

	// Get all products and warehouses
	let products = product_service.get_all_products().await.unwrap_or_default();
	let warehouses = warehouse_service.get_all_warehouses().await.unwrap_or_default();

	// Create lookup maps
	let product_map: std::collections::HashMap<_, _> = products.iter().map(|p| (p.id, p.clone())).collect();

	let warehouse_map: std::collections::HashMap<_, _> = warehouses.iter().map(|w| (w.id, w.clone())).collect();

	let stock_item_map: std::collections::HashMap<_, _> = stock_items.iter().map(|si| (si.id, si.clone())).collect();

	// Create transaction DTOs
	let transaction_dtos: Vec<TransactionDto> = transactions
		.iter()
		.filter_map(|t| {
			let stock_item = stock_item_map.get(&t.stock_item_id)?;
			let product = product_map.get(&stock_item.product_id)?;
			let warehouse = warehouse_map.get(&stock_item.warehouse_id)?;

			Some(TransactionDto {
				id: t.id,
				stock_item_id: t.stock_item_id,
				product_name: product.name.clone(),
				warehouse_name: warehouse.name.clone(),
				quantity: t.quantity,
				transaction_type: t.transaction_type.to_string(),
				reference_number: t.reference_number.clone(),
				notes: t.notes.clone(),
				created_at: t.created_at,
				created_by: t.created_by.clone(),
			})
		})
		.collect();

	let template = TransactionListTemplate {
		transactions: transaction_dtos,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

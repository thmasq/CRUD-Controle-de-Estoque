use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use chrono::{Duration, Utc};
use std::collections::HashMap;

use crate::AppState;
use crate::dtos::dashboard::{DashboardTemplate, TransactionChartData, TransactionDataset, WarehouseChartData};

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

	// Prepare transaction chart data (last 14 days)
	let transaction_chart_data = prepare_transaction_chart_data(&transactions);

	// Prepare warehouse chart data
	let warehouse_chart_data = prepare_warehouse_chart_data(&stock_items, &warehouse_map);

	// Serialize chart data to JSON strings
	let transaction_chart_json =
		serde_json::to_string(&transaction_chart_data).unwrap_or_else(|_| r#"{"labels":[],"datasets":[]}"#.to_string());

	let warehouse_chart_json = serde_json::to_string(&warehouse_chart_data)
		.unwrap_or_else(|_| r#"{"labels":[],"data":[],"background_colors":[]}"#.to_string());

	// Create template
	let template = DashboardTemplate {
		product_count: products.len() as u64,
		category_count: categories.len() as u64,
		warehouse_count: warehouses.len() as u64,
		stock_item_count: stock_items.len() as u64,
		recent_transactions,
		low_stock_items,
		transaction_chart_json,
		warehouse_chart_json,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

fn prepare_transaction_chart_data(
	transactions: &[stock_domain::entities::stock_transaction::StockTransaction],
) -> TransactionChartData {
	let now = Utc::now();
	let start_date = now - Duration::days(13); // Last 14 days

	// Generate labels for the last 14 days
	let mut labels = Vec::new();
	let mut date_to_index = HashMap::new();

	for i in 0..14 {
		let date = (start_date + Duration::days(i)).date_naive();
		let label = date.format("%m/%d").to_string();
		labels.push(label);
		date_to_index.insert(date, i as usize);
	}

	// Initialize data arrays
	let mut in_data = vec![0u32; 14];
	let mut out_data = vec![0u32; 14];
	let mut adjustment_data = vec![0u32; 14];

	// Group transactions by date and type
	for transaction in transactions {
		let transaction_date = transaction.created_at.date_naive();

		if let Some(&index) = date_to_index.get(&transaction_date) {
			match transaction.transaction_type {
				stock_domain::entities::stock_transaction::TransactionType::In => {
					in_data[index] += transaction.quantity.unsigned_abs();
				},
				stock_domain::entities::stock_transaction::TransactionType::Out => {
					out_data[index] += transaction.quantity.unsigned_abs();
				},
				stock_domain::entities::stock_transaction::TransactionType::Adjustment => {
					adjustment_data[index] += transaction.quantity.unsigned_abs();
				},
			}
		}
	}

	let datasets = vec![
		TransactionDataset {
			label: "Stock In".to_string(),
			data: in_data,
			border_color: "rgb(34, 197, 94)".to_string(),
			background_color: "rgba(34, 197, 94, 0.1)".to_string(),
			tension: 0.3,
		},
		TransactionDataset {
			label: "Stock Out".to_string(),
			data: out_data,
			border_color: "rgb(239, 68, 68)".to_string(),
			background_color: "rgba(239, 68, 68, 0.1)".to_string(),
			tension: 0.3,
		},
		TransactionDataset {
			label: "Adjustments".to_string(),
			data: adjustment_data,
			border_color: "rgb(168, 85, 247)".to_string(),
			background_color: "rgba(168, 85, 247, 0.1)".to_string(),
			tension: 0.3,
		},
	];

	TransactionChartData { labels, datasets }
}

fn prepare_warehouse_chart_data(
	stock_items: &[stock_domain::entities::stock_item::StockItem],
	warehouse_map: &HashMap<uuid::Uuid, stock_domain::entities::warehouse::Warehouse>,
) -> WarehouseChartData {
	let mut warehouse_totals: HashMap<String, u32> = HashMap::new();

	// Sum up stock quantities by warehouse
	for item in stock_items {
		if let Some(warehouse) = warehouse_map.get(&item.warehouse_id) {
			*warehouse_totals.entry(warehouse.name.clone()).or_insert(0) += item.quantity as u32;
		}
	}

	if warehouse_totals.is_empty() {
		for warehouse in warehouse_map.values() {
			warehouse_totals.insert(warehouse.name.clone(), 0);
		}
	}

	// Sort warehouses by stock quantity (descending)
	let mut sorted_warehouses: Vec<_> = warehouse_totals.into_iter().collect();
	sorted_warehouses.sort_by(|a, b| b.1.cmp(&a.1));

	let labels: Vec<String> = sorted_warehouses.iter().map(|(name, _)| name.clone()).collect();
	let data: Vec<u32> = sorted_warehouses.iter().map(|(_, quantity)| *quantity).collect();

	// Generate colors for warehouses
	let colors = [
		"rgba(59, 130, 246, 0.8)".to_string(), // Blue
		"rgba(34, 197, 94, 0.8)".to_string(),  // Green
		"rgba(239, 68, 68, 0.8)".to_string(),  // Red
		"rgba(168, 85, 247, 0.8)".to_string(), // Purple
		"rgba(245, 158, 11, 0.8)".to_string(), // Amber
		"rgba(236, 72, 153, 0.8)".to_string(), // Pink
		"rgba(20, 184, 166, 0.8)".to_string(), // Teal
		"rgba(99, 102, 241, 0.8)".to_string(),
	];

	let background_colors = labels
		.iter()
		.enumerate()
		.map(|(i, _)| colors.get(i % colors.len()).unwrap_or(&colors[0]).clone())
		.collect();

	WarehouseChartData {
		labels,
		data,
		background_colors,
	}
}

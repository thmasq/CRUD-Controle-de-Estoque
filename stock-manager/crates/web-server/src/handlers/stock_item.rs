use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use uuid::Uuid;

use stock_application::services::stock_item_service::{StockItemCreateDto, StockItemUpdateDto};
use stock_domain::entities::stock_transaction::TransactionType;

use crate::AppState;
use crate::dtos::product::ProductDto;
use crate::dtos::stock_item::{
	StockItemCreateRequest, StockItemDto, StockItemFilterRequest, StockItemFormTemplate, StockItemListTemplate,
	StockItemTransactionTemplate, StockItemUpdateRequest,
};
use crate::dtos::stock_transaction::TransactionCreateRequest;
use crate::dtos::warehouse::WarehouseDto;

pub async fn list_stock_items(
	state: web::Data<AppState>,
	query: web::Query<StockItemFilterRequest>,
) -> Result<HttpResponse> {
	let stock_service = state.stock_item_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	// Get all data
	let mut stock_items = stock_service.get_all_stock_items().await.unwrap_or_default();
	let products = product_service.get_all_products().await.unwrap_or_default();
	let warehouses = warehouse_service.get_all_warehouses().await.unwrap_or_default();

	// Create lookup maps
	let product_map: std::collections::HashMap<_, _> = products.iter().map(|p| (p.id, p.clone())).collect();

	let warehouse_map: std::collections::HashMap<_, _> = warehouses.iter().map(|w| (w.id, w.clone())).collect();

	// Filter stock items if needed
	if let Some(product_id) = query.product_id {
		stock_items = stock_items
			.into_iter()
			.filter(|si| si.product_id == product_id)
			.collect();
	}

	if let Some(warehouse_id) = query.warehouse_id {
		stock_items = stock_items
			.into_iter()
			.filter(|si| si.warehouse_id == warehouse_id)
			.collect();
	}

	// Create DTOs
	let stock_item_dtos: Vec<StockItemDto> = stock_items
		.iter()
		.filter_map(|si| {
			let product = product_map.get(&si.product_id)?;
			let warehouse = warehouse_map.get(&si.warehouse_id)?;

			Some(StockItemDto {
				id: si.id,
				product_id: si.product_id,
				product_name: product.name.clone(),
				warehouse_id: si.warehouse_id,
				warehouse_name: warehouse.name.clone(),
				quantity: si.quantity,
				unit_cost: si.unit_cost,
				last_restocked: si.last_restocked,
				is_active: si.is_active,
			})
		})
		.collect();

	// Create product and warehouse DTOs for filters
	let product_dtos: Vec<ProductDto> = products
		.iter()
		.map(|p| ProductDto {
			id: p.id,
			name: p.name.clone(),
			description: p.description.clone(),
			sku: p.sku.clone(),
			category_id: p.category_id,
			category_name: None, // Not needed for dropdown
			is_active: p.is_active,
		})
		.collect();

	let warehouse_dtos: Vec<WarehouseDto> = warehouses
		.iter()
		.map(|w| WarehouseDto {
			id: w.id,
			name: w.name.clone(),
			location: w.location.clone(),
			contact_info: w.contact_info.clone(),
			is_active: w.is_active,
		})
		.collect();

	let template = StockItemListTemplate {
		stock_items: stock_item_dtos,
		products: product_dtos,
		warehouses: warehouse_dtos,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn new_stock_item_form(state: web::Data<AppState>) -> Result<HttpResponse> {
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	let products = product_service.get_all_products().await.unwrap_or_default();
	let warehouses = warehouse_service.get_all_warehouses().await.unwrap_or_default();

	// Create DTOs
	let product_dtos: Vec<ProductDto> = products
		.iter()
		.filter(|p| p.is_active) // Only show active products
		.map(|p| ProductDto {
			id: p.id,
			name: p.name.clone(),
			description: p.description.clone(),
			sku: p.sku.clone(),
			category_id: p.category_id,
			category_name: None,
			is_active: p.is_active,
		})
		.collect();

	let warehouse_dtos: Vec<WarehouseDto> = warehouses
		.iter()
		.filter(|w| w.is_active) // Only show active warehouses
		.map(|w| WarehouseDto {
			id: w.id,
			name: w.name.clone(),
			location: w.location.clone(),
			contact_info: w.contact_info.clone(),
			is_active: w.is_active,
		})
		.collect();

	let template = StockItemFormTemplate {
		form_title: "Add Stock Item".to_string(),
		form_action: "/stock-items".to_string(),
		form_method: "post".to_string(),
		stock_item: StockItemDto {
			id: Uuid::nil(),
			product_id: Uuid::nil(),
			product_name: "".to_string(),
			warehouse_id: Uuid::nil(),
			warehouse_name: "".to_string(),
			quantity: 0,
			unit_cost: rust_decimal::Decimal::new(0, 0),
			last_restocked: chrono::Utc::now(),
			is_active: true,
		},
		products: product_dtos,
		warehouses: warehouse_dtos,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn edit_stock_item_form(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let stock_item_id = path.into_inner();
	let stock_service = state.stock_item_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	if let Some(stock_item) = stock_service.get_stock_item(stock_item_id).await.unwrap_or(None) {
		let products = product_service.get_all_products().await.unwrap_or_default();
		let warehouses = warehouse_service.get_all_warehouses().await.unwrap_or_default();

		// Get product and warehouse info
		let product = products
			.iter()
			.find(|p| p.id == stock_item.product_id)
			.ok_or_else(|| actix_web::error::ErrorNotFound("Product not found"))?;

		let warehouse = warehouses
			.iter()
			.find(|w| w.id == stock_item.warehouse_id)
			.ok_or_else(|| actix_web::error::ErrorNotFound("Warehouse not found"))?;

		// Create DTOs
		let product_dtos: Vec<ProductDto> = products
			.iter()
			.map(|p| ProductDto {
				id: p.id,
				name: p.name.clone(),
				description: p.description.clone(),
				sku: p.sku.clone(),
				category_id: p.category_id,
				category_name: None,
				is_active: p.is_active,
			})
			.collect();

		let warehouse_dtos: Vec<WarehouseDto> = warehouses
			.iter()
			.map(|w| WarehouseDto {
				id: w.id,
				name: w.name.clone(),
				location: w.location.clone(),
				contact_info: w.contact_info.clone(),
				is_active: w.is_active,
			})
			.collect();

		let template = StockItemFormTemplate {
			form_title: "Edit Stock Item".to_string(),
			form_action: format!("/stock-items/{}", stock_item_id),
			form_method: "put".to_string(),
			stock_item: StockItemDto {
				id: stock_item.id,
				product_id: stock_item.product_id,
				product_name: product.name.clone(),
				warehouse_id: stock_item.warehouse_id,
				warehouse_name: warehouse.name.clone(),
				quantity: stock_item.quantity,
				unit_cost: stock_item.unit_cost,
				last_restocked: stock_item.last_restocked,
				is_active: stock_item.is_active,
			},
			products: product_dtos,
			warehouses: warehouse_dtos,
		};

		Ok(HttpResponse::Ok()
			.content_type("text/html")
			.body(template.dyn_render().unwrap()))
	} else {
		Ok(HttpResponse::NotFound().body("Stock item not found"))
	}
}

pub async fn transaction_form(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let stock_item_id = path.into_inner();
	let stock_service = state.stock_item_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	if let Some(stock_item) = stock_service.get_stock_item(stock_item_id).await.unwrap_or(None) {
		let product = product_service
			.get_product(stock_item.product_id)
			.await
			.unwrap_or(None)
			.ok_or_else(|| actix_web::error::ErrorNotFound("Product not found"))?;

		let warehouse = warehouse_service
			.get_warehouse(stock_item.warehouse_id)
			.await
			.unwrap_or(None)
			.ok_or_else(|| actix_web::error::ErrorNotFound("Warehouse not found"))?;

		let template = StockItemTransactionTemplate {
			form_action: format!("/stock-items/{}/transaction", stock_item_id),
			stock_item: StockItemDto {
				id: stock_item.id,
				product_id: stock_item.product_id,
				product_name: product.name,
				warehouse_id: stock_item.warehouse_id,
				warehouse_name: warehouse.name,
				quantity: stock_item.quantity,
				unit_cost: stock_item.unit_cost,
				last_restocked: stock_item.last_restocked,
				is_active: stock_item.is_active,
			},
		};

		Ok(HttpResponse::Ok()
			.content_type("text/html")
			.body(template.dyn_render().unwrap()))
	} else {
		Ok(HttpResponse::NotFound().body("Stock item not found"))
	}
}

pub async fn create_stock_item(
	state: web::Data<AppState>,
	form: web::Form<StockItemCreateRequest>,
) -> Result<HttpResponse> {
	let stock_service = state.stock_item_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	let dto = StockItemCreateDto {
		product_id: form.product_id,
		warehouse_id: form.warehouse_id,
		quantity: form.quantity,
		unit_cost: form.unit_cost,
	};

	match stock_service.create_stock_item(dto).await {
		Ok(stock_item) => {
			// Get product and warehouse info
			let product = product_service
				.get_product(stock_item.product_id)
				.await
				.unwrap_or(None)
				.ok_or_else(|| actix_web::error::ErrorNotFound("Product not found"))?;

			let warehouse = warehouse_service
				.get_warehouse(stock_item.warehouse_id)
				.await
				.unwrap_or(None)
				.ok_or_else(|| actix_web::error::ErrorNotFound("Warehouse not found"))?;

			// Return row HTML for the new stock item
			let stock_item_dto = StockItemDto {
				id: stock_item.id,
				product_id: stock_item.product_id,
				product_name: product.name,
				warehouse_id: stock_item.warehouse_id,
				warehouse_name: warehouse.name,
				quantity: stock_item.quantity,
				unit_cost: stock_item.unit_cost,
				last_restocked: stock_item.last_restocked,
				is_active: stock_item.is_active,
			};

			let html = format!(
				r##"<tr id="stock-item-row-{id}">
                    <td class="px-6 py-4 whitespace-nowrap">{product_name}</td>
                    <td class="px-6 py-4 whitespace-nowrap">{warehouse_name}</td>
                    <td class="px-6 py-4 whitespace-nowrap">{quantity}</td>
                    <td class="px-6 py-4 whitespace-nowrap">${unit_cost}</td>
                    <td class="px-6 py-4 whitespace-nowrap">{last_restocked}</td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">
                        <button 
                            class="text-indigo-600 hover:text-indigo-900 mr-3"
                            hx-get="/stock-items/{id}/edit"
                            hx-target="#modal-container"
                            hx-trigger="click">
                            Edit
                        </button>
                        <button 
                            class="text-green-600 hover:text-green-900 mr-3"
                            hx-get="/stock-items/{id}/transaction"
                            hx-target="#modal-container"
                            hx-trigger="click">
                            Transaction
                        </button>
                        <button 
                            class="text-red-600 hover:text-red-900"
                            hx-delete="/stock-items/{id}"
                            hx-confirm="Are you sure you want to delete this stock item?"
                            hx-target="#stock-item-row-{id}"
                            hx-swap="outerHTML"
                            hx-trigger="click">
                            Delete
                        </button>
                    </td>
                </tr>"##,
				id = stock_item_dto.id,
				product_name = stock_item_dto.product_name,
				warehouse_name = stock_item_dto.warehouse_name,
				quantity = stock_item_dto.quantity,
				unit_cost = stock_item_dto.unit_cost,
				last_restocked = stock_item_dto.last_restocked.format("%Y-%m-%d %H:%M")
			);

			Ok(HttpResponse::Ok()
				.append_header(("HX-Trigger", "itemCreated"))
				.content_type("text/html")
				.body(html))
		},
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to create stock item: {}", e))),
	}
}

pub async fn update_stock_item(
	path: web::Path<Uuid>,
	state: web::Data<AppState>,
	form: web::Form<StockItemUpdateRequest>,
) -> Result<HttpResponse> {
	let stock_item_id = path.into_inner();
	let stock_service = state.stock_item_service.clone();

	let dto = StockItemUpdateDto {
		id: stock_item_id,
		quantity: form.quantity,
		unit_cost: form.unit_cost,
	};

	match stock_service.update_stock_item(dto).await {
		Ok(_) => Ok(HttpResponse::Ok()
			.append_header(("HX-Trigger", "itemUpdated"))
			.append_header(("HX-Redirect", "/stock-items"))
			.finish()),
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to update stock item: {}", e))),
	}
}

pub async fn delete_stock_item(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let stock_item_id = path.into_inner();
	let stock_service = state.stock_item_service.clone();

	match stock_service.delete_stock_item(stock_item_id).await {
		Ok(true) => Ok(HttpResponse::Ok().append_header(("HX-Trigger", "itemDeleted")).finish()),
		Ok(false) => Ok(HttpResponse::NotFound().body("Stock item not found")),
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to delete stock item: {}", e))),
	}
}

pub async fn create_transaction(
	path: web::Path<Uuid>,
	state: web::Data<AppState>,
	form: web::Form<TransactionCreateRequest>,
) -> Result<HttpResponse> {
	let stock_item_id = path.into_inner();
	let transaction_service = state.transaction_service.clone();
	let stock_service = state.stock_item_service.clone();
	let product_service = state.product_service.clone();
	let warehouse_service = state.warehouse_service.clone();

	// Convert transaction type string to enum
	let transaction_type = match form.transaction_type.as_str() {
		"IN" => TransactionType::In,
		"OUT" => TransactionType::Out,
		"ADJUSTMENT" => TransactionType::Adjustment,
		_ => return Ok(HttpResponse::BadRequest().body("Invalid transaction type")),
	};

	// Create transaction
	let dto = stock_application::services::stock_transaction_service::StockTransactionCreateDto {
		stock_item_id,
		quantity: form.quantity,
		transaction_type,
		reference_number: form.reference_number.clone(),
		notes: form.notes.clone(),
		created_by: form.created_by.clone(),
	};

	match transaction_service.create_transaction(dto).await {
		Ok(_) => {
			// Get updated stock item info for the row
			if let Some(stock_item) = stock_service.get_stock_item(stock_item_id).await.unwrap_or(None) {
				let product = product_service
					.get_product(stock_item.product_id)
					.await
					.unwrap_or(None)
					.ok_or_else(|| actix_web::error::ErrorNotFound("Product not found"))?;

				let warehouse = warehouse_service
					.get_warehouse(stock_item.warehouse_id)
					.await
					.unwrap_or(None)
					.ok_or_else(|| actix_web::error::ErrorNotFound("Warehouse not found"))?;

				// Return updated stock item row
				let stock_item_dto = StockItemDto {
					id: stock_item.id,
					product_id: stock_item.product_id,
					product_name: product.name,
					warehouse_id: stock_item.warehouse_id,
					warehouse_name: warehouse.name,
					quantity: stock_item.quantity,
					unit_cost: stock_item.unit_cost,
					last_restocked: stock_item.last_restocked,
					is_active: stock_item.is_active,
				};

				let html = format!(
					r##"<tr id="stock-item-row-{id}">
                        <td class="px-6 py-4 whitespace-nowrap">{product_name}</td>
                        <td class="px-6 py-4 whitespace-nowrap">{warehouse_name}</td>
                        <td class="px-6 py-4 whitespace-nowrap">{quantity}</td>
                        <td class="px-6 py-4 whitespace-nowrap">${unit_cost}</td>
                        <td class="px-6 py-4 whitespace-nowrap">{last_restocked}</td>
                        <td class="px-6 py-4 whitespace-nowrap text-sm">
                            <button 
                                class="text-indigo-600 hover:text-indigo-900 mr-3"
                                hx-get="/stock-items/{id}/edit"
                                hx-target="#modal-container"
                                hx-trigger="click">
                                Edit
                            </button>
                            <button 
                                class="text-green-600 hover:text-green-900 mr-3"
                                hx-get="/stock-items/{id}/transaction"
                                hx-target="#modal-container"
                                hx-trigger="click">
                                Transaction
                            </button>
                            <button 
                                class="text-red-600 hover:text-red-900"
                                hx-delete="/stock-items/{id}"
                                hx-confirm="Are you sure you want to delete this stock item?"
                                hx-target="#stock-item-row-{id}"
                                hx-swap="outerHTML"
                                hx-trigger="click">
                                Delete
                            </button>
                        </td>
                    </tr>"##,
					id = stock_item_dto.id,
					product_name = stock_item_dto.product_name,
					warehouse_name = stock_item_dto.warehouse_name,
					quantity = stock_item_dto.quantity,
					unit_cost = stock_item_dto.unit_cost,
					last_restocked = stock_item_dto.last_restocked.format("%Y-%m-%d %H:%M")
				);

				Ok(HttpResponse::Ok()
					.append_header(("HX-Trigger", "itemUpdated"))
					.content_type("text/html")
					.body(html))
			} else {
				Ok(HttpResponse::NotFound().body("Stock item not found"))
			}
		},
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to create transaction: {}", e))),
	}
}

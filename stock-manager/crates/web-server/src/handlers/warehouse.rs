use actix_web::{HttpResponse, Result, web};
use askama::DynTemplate;
use uuid::Uuid;

use stock_application::services::warehouse_service::{WarehouseCreateDto, WarehouseUpdateDto};

use crate::AppState;
use crate::dtos::warehouse::{
	WarehouseCreateRequest, WarehouseDto, WarehouseFormTemplate, WarehouseListTemplate, WarehouseUpdateRequest,
};

pub async fn list_warehouses(state: web::Data<AppState>) -> Result<HttpResponse> {
	let service = state.warehouse_service.clone();
	let warehouses = service.get_all_warehouses().await.unwrap_or_default();

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

	let template = WarehouseListTemplate {
		warehouses: warehouse_dtos,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn new_warehouse_form() -> Result<HttpResponse> {
	let template = WarehouseFormTemplate {
		form_title: "Add Warehouse".to_string(),
		form_action: "/warehouses".to_string(),
		form_method: "post".to_string(),
		warehouse: WarehouseDto {
			id: Uuid::nil(),
			name: String::new(),
			location: String::new(),
			contact_info: None,
			is_active: true,
		},
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn edit_warehouse_form(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let warehouse_id = path.into_inner();
	let service = state.warehouse_service.clone();

	if let Some(warehouse) = service.get_warehouse(warehouse_id).await.unwrap_or(None) {
		let template = WarehouseFormTemplate {
			form_title: "Edit Warehouse".to_string(),
			form_action: format!("/warehouses/{warehouse_id}"),
			form_method: "put".to_string(),
			warehouse: WarehouseDto {
				id: warehouse.id,
				name: warehouse.name,
				location: warehouse.location,
				contact_info: warehouse.contact_info,
				is_active: warehouse.is_active,
			},
		};

		Ok(HttpResponse::Ok()
			.content_type("text/html")
			.body(template.dyn_render().unwrap()))
	} else {
		Ok(HttpResponse::NotFound().body("Warehouse not found"))
	}
}

pub async fn create_warehouse(
	state: web::Data<AppState>,
	form: web::Form<WarehouseCreateRequest>,
) -> Result<HttpResponse> {
	let service = state.warehouse_service.clone();

	let dto = WarehouseCreateDto {
		name: form.name.clone(),
		location: form.location.clone(),
		contact_info: form.contact_info.clone(),
		is_active: form.is_active,
	};

	match service.create_warehouse(dto).await {
		Ok(warehouse) => {
			// Return row HTML for the new warehouse
			let warehouse_dto = WarehouseDto {
				id: warehouse.id,
				name: warehouse.name,
				location: warehouse.location,
				contact_info: warehouse.contact_info,
				is_active: warehouse.is_active,
			};

			let status_class = if warehouse_dto.is_active {
				"bg-green-100 text-green-800"
			} else {
				"bg-red-100 text-red-800"
			};

			let status_text = if warehouse_dto.is_active { "Active" } else { "Inactive" };

			let html = format!(
				r##"<tr id="warehouse-row-{id}">
                    <td class="px-6 py-4 whitespace-nowrap">{name}</td>
                    <td class="px-6 py-4">{location}</td>
                    <td class="px-6 py-4">{contact_info}</td>
                    <td class="px-6 py-4 whitespace-nowrap">
                        <span class="px-2 py-1 text-xs rounded-full {status_class}">
                            {status_text}
                        </span>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">
                        <button 
                            class="text-indigo-600 hover:text-indigo-900 mr-3"
                            hx-get="/warehouses/{id}/edit"
                            hx-target="#modal-container"
                            hx-trigger="click">
                            Edit
                        </button>
                        <button 
                            class="text-red-600 hover:text-red-900"
                            hx-delete="/warehouses/{id}"
                            hx-confirm="Are you sure you want to delete this warehouse?"
                            hx-target="#warehouse-row-{id}"
                            hx-swap="outerHTML"
                            hx-trigger="click">
                            Delete
                        </button>
                    </td>
                </tr>"##,
				id = warehouse_dto.id,
				name = warehouse_dto.name,
				location = warehouse_dto.location,
				contact_info = warehouse_dto.contact_info.unwrap_or_else(|| "--".to_string()),
				status_class = status_class,
				status_text = status_text
			);

			Ok(HttpResponse::Ok()
				.append_header(("HX-Trigger", "itemCreated"))
				.content_type("text/html")
				.body(html))
		},
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to create warehouse: {e}"))),
	}
}

pub async fn update_warehouse(
	path: web::Path<Uuid>,
	state: web::Data<AppState>,
	form: web::Form<WarehouseUpdateRequest>,
) -> Result<HttpResponse> {
	let warehouse_id = path.into_inner();
	let service = state.warehouse_service.clone();

	let dto = WarehouseUpdateDto {
		id: warehouse_id,
		name: form.name.clone(),
		location: form.location.clone(),
		contact_info: form.contact_info.clone(),
		is_active: form.is_active,
	};

	match service.update_warehouse(dto).await {
		Ok(_) => Ok(HttpResponse::Ok()
			.append_header(("HX-Trigger", "itemUpdated"))
			.append_header(("HX-Redirect", "/warehouses"))
			.finish()),
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to update warehouse: {e}"))),
	}
}

pub async fn delete_warehouse(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let warehouse_id = path.into_inner();
	let service = state.warehouse_service.clone();

	match service.delete_warehouse(warehouse_id).await {
		Ok(true) => Ok(HttpResponse::Ok().append_header(("HX-Trigger", "itemDeleted")).finish()),
		Ok(false) => Ok(HttpResponse::NotFound().body("Warehouse not found")),
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to delete warehouse: {e}"))),
	}
}

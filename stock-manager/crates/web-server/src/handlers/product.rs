use actix_web::{HttpRequest, HttpResponse, Result, web};
use askama::DynTemplate;
use uuid::Uuid;

use stock_application::services::product_service::{ProductCreateDto, ProductUpdateDto};

use crate::AppState;
use crate::dtos::category::CategoryDto;
use crate::dtos::product::{
	ProductCreateRequest, ProductDto, ProductFilterRequest, ProductFormTemplate, ProductListTemplate,
	ProductTableRowsTemplate, ProductUpdateRequest,
};

pub async fn list_products(
	state: web::Data<AppState>,
	req: HttpRequest,
	query: web::Query<ProductFilterRequest>,
) -> Result<HttpResponse> {
	let product_service = state.product_service.clone();
	let category_service = state.category_service.clone();

	// Get products
	let mut products = product_service.get_all_products().await.unwrap_or_default();

	// Filter by category if specified
	if let Some(category_id) = query.category_id {
		products.retain(|p| p.category_id.is_some() && p.category_id.unwrap() == category_id);
	}

	// Filter by status if specified
	if let Some(ref status) = query.status {
		// Only filter if status is not empty
		if !status.is_empty() {
			let is_active = status == "active";
			products.retain(|p| p.is_active == is_active);
		}
	}

	// Get categories
	let categories = category_service.get_all_categories().await.unwrap_or_default();
	let category_map: std::collections::HashMap<_, _> = categories.iter().map(|c| (c.id, c.name.clone())).collect();

	// Create DTOs
	let mut product_dtos = Vec::new();
	for product in products {
		let category_name = product.category_id.and_then(|id| category_map.get(&id).cloned());

		product_dtos.push(ProductDto {
			id: product.id,
			name: product.name,
			description: product.description,
			sku: product.sku,
			category_id: product.category_id,
			category_name,
			is_active: product.is_active,
		});
	}

	let is_htmx_request = req.headers().contains_key("HX-Request");
	drop(req);

	if is_htmx_request {
		// Return just the table rows
		let rows_template = ProductTableRowsTemplate { products: product_dtos };

		Ok(HttpResponse::Ok()
			.content_type("text/html")
			.body(rows_template.dyn_render().unwrap()))
	} else {
		// Create category DTOs
		let category_dtos: Vec<CategoryDto> = categories
			.iter()
			.map(|c| CategoryDto {
				id: c.id,
				name: c.name.clone(),
				description: c.description.clone(),
				product_count: 0,
			})
			.collect();

		let template = ProductListTemplate {
			products: product_dtos,
			categories: category_dtos,
		};

		Ok(HttpResponse::Ok()
			.content_type("text/html")
			.body(template.dyn_render().unwrap()))
	}
}

pub async fn new_product_form(state: web::Data<AppState>) -> Result<HttpResponse> {
	let category_service = state.category_service.clone();
	let categories = category_service.get_all_categories().await.unwrap_or_default();

	let category_dtos: Vec<CategoryDto> = categories
		.iter()
		.map(|c| CategoryDto {
			id: c.id,
			name: c.name.clone(),
			description: c.description.clone(),
			product_count: 0,
		})
		.collect();

	let template = ProductFormTemplate {
		form_title: "Add Product".to_string(),
		form_action: "/products".to_string(),
		form_method: "post".to_string(),
		product: ProductDto {
			id: Uuid::nil(),
			name: String::new(),
			description: None,
			sku: String::new(),
			category_id: None,
			category_name: None,
			is_active: true,
		},
		categories: category_dtos,
	};

	Ok(HttpResponse::Ok()
		.content_type("text/html")
		.body(template.dyn_render().unwrap()))
}

pub async fn edit_product_form(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let product_id = path.into_inner();
	let product_service = state.product_service.clone();
	let category_service = state.category_service.clone();

	if let Some(product) = product_service.get_product(product_id).await.unwrap_or(None) {
		// Get categories
		let categories = category_service.get_all_categories().await.unwrap_or_default();
		let category_map: std::collections::HashMap<_, _> = categories.iter().map(|c| (c.id, c.name.clone())).collect();

		let category_name = product.category_id.and_then(|id| category_map.get(&id).cloned());

		let category_dtos: Vec<CategoryDto> = categories
			.iter()
			.map(|c| CategoryDto {
				id: c.id,
				name: c.name.clone(),
				description: c.description.clone(),
				product_count: 0,
			})
			.collect();

		let template = ProductFormTemplate {
			form_title: "Edit Product".to_string(),
			form_action: format!("/products/{product_id}"),
			form_method: "put".to_string(),
			product: ProductDto {
				id: product.id,
				name: product.name,
				description: product.description,
				sku: product.sku,
				category_id: product.category_id,
				category_name,
				is_active: product.is_active,
			},
			categories: category_dtos,
		};

		Ok(HttpResponse::Ok()
			.content_type("text/html")
			.body(template.dyn_render().unwrap()))
	} else {
		Ok(HttpResponse::NotFound().body("Product not found"))
	}
}

pub async fn create_product(state: web::Data<AppState>, form: web::Form<ProductCreateRequest>) -> Result<HttpResponse> {
	let service = state.product_service.clone();

	let dto = ProductCreateDto {
		name: form.name.clone(),
		description: form.description.clone(),
		sku: form.sku.clone(),
		category_id: form.category_id,
		is_active: form.is_active,
	};

	match service.create_product(dto).await {
		Ok(product) => {
			// Get category name if needed
			let mut category_name = None;
			if let Some(category_id) = product.category_id
				&& let Some(category) = state.category_service.get_category(category_id).await.unwrap_or(None)
			{
				category_name = Some(category.name);
			}

			// Return row HTML for the new product
			let product_dto = ProductDto {
				id: product.id,
				name: product.name,
				description: product.description,
				sku: product.sku,
				category_id: product.category_id,
				category_name,
				is_active: product.is_active,
			};

			let status_class = if product_dto.is_active {
				"bg-green-100 text-green-800"
			} else {
				"bg-red-100 text-red-800"
			};

			let status_text = if product_dto.is_active { "Active" } else { "Inactive" };

			let html = format!(
				r##"<tr id="product-row-{id}">
                    <td class="px-6 py-4 whitespace-nowrap font-mono">{sku}</td>
                    <td class="px-6 py-4 whitespace-nowrap">{name}</td>
                    <td class="px-6 py-4 whitespace-nowrap">{category}</td>
                    <td class="px-6 py-4 whitespace-nowrap">
                        <span class="px-2 py-1 text-xs rounded-full {status_class}">
                            {status_text}
                        </span>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-sm">
                        <button 
                            class="text-indigo-600 hover:text-indigo-900 mr-3"
                            hx-get="/products/{id}/edit"
                            hx-target="#modal-container"
                            hx-trigger="click">
                            Edit
                        </button>
                        <button 
                            class="text-red-600 hover:text-red-900"
                            hx-delete="/products/{id}"
                            hx-confirm="Are you sure you want to delete this product?"
                            hx-target="#product-row-{id}"
                            hx-swap="outerHTML"
                            hx-trigger="click">
                            Delete
                        </button>
                    </td>
                </tr>"##,
				id = product_dto.id,
				name = product_dto.name,
				sku = product_dto.sku,
				category = product_dto.category_name.unwrap_or_else(|| "--".to_string()),
				status_class = status_class,
				status_text = status_text
			);

			Ok(HttpResponse::Ok()
				.append_header(("HX-Trigger", "itemCreated"))
				.content_type("text/html")
				.body(html))
		},
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to create product: {e}"))),
	}
}

pub async fn update_product(
	path: web::Path<Uuid>,
	state: web::Data<AppState>,
	form: web::Form<ProductUpdateRequest>,
) -> Result<HttpResponse> {
	let product_id = path.into_inner();
	let service = state.product_service.clone();

	let dto = ProductUpdateDto {
		id: product_id,
		name: form.name.clone(),
		description: form.description.clone(),
		sku: form.sku.clone(),
		category_id: form.category_id,
		is_active: form.is_active,
	};

	match service.update_product(dto).await {
		Ok(_) => Ok(HttpResponse::Ok()
			.append_header(("HX-Trigger", "itemUpdated"))
			.append_header(("HX-Redirect", "/products"))
			.finish()),
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to update product: {e}"))),
	}
}

pub async fn delete_product(path: web::Path<Uuid>, state: web::Data<AppState>) -> Result<HttpResponse> {
	let product_id = path.into_inner();
	let service = state.product_service.clone();

	match service.delete_product(product_id).await {
		Ok(true) => Ok(HttpResponse::Ok().append_header(("HX-Trigger", "itemDeleted")).finish()),
		Ok(false) => Ok(HttpResponse::NotFound().body("Product not found")),
		Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to delete product: {e}"))),
	}
}

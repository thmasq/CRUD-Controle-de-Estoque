use askama::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "products/index.html")]
pub struct ProductListTemplate {
	pub products: Vec<ProductDto>,
	pub categories: Vec<super::category::CategoryDto>,
}

#[derive(Template)]
#[template(path = "products/form.html")]
pub struct ProductFormTemplate {
	pub form_title: String,
	pub form_action: String,
	pub form_method: String,
	pub product: ProductDto,
	pub categories: Vec<super::category::CategoryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDto {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub category_name: Option<String>,
	pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProductCreateRequest {
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProductUpdateRequest {
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProductFilterRequest {
	pub category_id: Option<Uuid>,
	pub status: Option<String>,
}

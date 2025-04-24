use askama::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "categories/index.html")]
pub struct CategoryListTemplate {
	pub categories: Vec<CategoryDto>,
}

#[derive(Template)]
#[template(path = "categories/form.html")]
pub struct CategoryFormTemplate {
	pub form_title: String,
	pub form_action: String,
	pub form_method: String,
	pub category: CategoryDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDto {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub product_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CategoryCreateRequest {
	pub name: String,
	pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryUpdateRequest {
	pub name: String,
	pub description: Option<String>,
}

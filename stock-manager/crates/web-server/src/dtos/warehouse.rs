use askama::Template;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "warehouses/index.html")]
pub struct WarehouseListTemplate {
	pub warehouses: Vec<WarehouseDto>,
}

#[derive(Template)]
#[template(path = "warehouses/form.html")]
pub struct WarehouseFormTemplate {
	pub form_title: String,
	pub form_action: String,
	pub form_method: String,
	pub warehouse: WarehouseDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseDto {
	pub id: Uuid,
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct WarehouseCreateRequest {
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct WarehouseUpdateRequest {
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

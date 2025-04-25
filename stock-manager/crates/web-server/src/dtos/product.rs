use crate::filters;
use askama::Template;
use serde::{Deserialize, Deserializer, Serialize};
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

fn deserialize_optional_uuid<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Option::<String>::deserialize(deserializer)?;
	match s {
		None => Ok(None),
		Some(s) if s.is_empty() => Ok(None),
		Some(s) => match Uuid::parse_str(&s) {
			Ok(id) => Ok(Some(id)),
			Err(e) => Err(serde::de::Error::custom(e)),
		},
	}
}

#[derive(Debug, Deserialize)]
pub struct ProductFilterRequest {
	#[serde(deserialize_with = "deserialize_optional_uuid", default)]
	pub category_id: Option<Uuid>,
	pub status: Option<String>,
}

#[derive(Template)]
#[template(path = "products/table_rows.html")]
pub struct ProductTableRowsTemplate {
	pub products: Vec<ProductDto>,
}

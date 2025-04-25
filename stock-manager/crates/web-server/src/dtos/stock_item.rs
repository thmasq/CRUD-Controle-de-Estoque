use crate::filters;
use askama::Template;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "stock_items/index.html")]
pub struct StockItemListTemplate {
	pub stock_items: Vec<StockItemDto>,
	pub warehouses: Vec<super::warehouse::WarehouseDto>,
	pub products: Vec<super::product::ProductDto>,
}

#[derive(Template)]
#[template(path = "stock_items/form.html")]
pub struct StockItemFormTemplate {
	pub form_title: String,
	pub form_action: String,
	pub form_method: String,
	pub stock_item: StockItemDto,
	pub warehouses: Vec<super::warehouse::WarehouseDto>,
	pub products: Vec<super::product::ProductDto>,
}

#[derive(Template)]
#[template(path = "stock_items/transaction_form.html")]
pub struct StockItemTransactionTemplate {
	pub form_action: String,
	pub stock_item: StockItemDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockItemDto {
	pub id: Uuid,
	pub product_id: Uuid,
	pub product_name: String,
	pub warehouse_id: Uuid,
	pub warehouse_name: String,
	pub quantity: i32,
	pub unit_cost: Decimal,
	pub last_restocked: DateTime<Utc>,
	pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct StockItemCreateRequest {
	pub product_id: Uuid,
	pub warehouse_id: Uuid,
	pub quantity: i32,
	pub unit_cost: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct StockItemUpdateRequest {
	pub quantity: i32,
	pub unit_cost: Decimal,
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
pub struct StockItemFilterRequest {
	#[serde(deserialize_with = "deserialize_optional_uuid", default)]
	pub warehouse_id: Option<Uuid>,
	#[serde(deserialize_with = "deserialize_optional_uuid", default)]
	pub product_id: Option<Uuid>,
}

#[derive(Template)]
#[template(path = "stock_items/table_rows.html")]
pub struct StockItemTableRowsTemplate {
	pub stock_items: Vec<StockItemDto>,
}

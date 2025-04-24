use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rust_decimal::Decimal;
use stock_domain::entities::stock_item::StockItem;
use uuid::Uuid;

use crate::schema::stock_items;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = stock_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StockItemModel {
	pub id: Uuid,
	pub product_id: Uuid,
	pub warehouse_id: Uuid,
	pub quantity: i32,
	pub unit_cost: Decimal,
	pub last_restocked: DateTime<Utc>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = stock_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStockItemModel {
	pub id: Uuid,
	pub product_id: Uuid,
	pub warehouse_id: Uuid,
	pub quantity: i32,
	pub unit_cost: Decimal,
	pub last_restocked: DateTime<Utc>,
}

impl From<StockItemModel> for StockItem {
	fn from(model: StockItemModel) -> Self {
		Self {
			id: model.id,
			product_id: model.product_id,
			warehouse_id: model.warehouse_id,
			quantity: model.quantity,
			unit_cost: model.unit_cost,
			last_restocked: model.last_restocked,
			created_at: model.created_at,
			updated_at: model.updated_at,
		}
	}
}

impl From<StockItem> for NewStockItemModel {
	fn from(entity: StockItem) -> Self {
		Self {
			id: entity.id,
			product_id: entity.product_id,
			warehouse_id: entity.warehouse_id,
			quantity: entity.quantity,
			unit_cost: entity.unit_cost,
			last_restocked: entity.last_restocked,
		}
	}
}

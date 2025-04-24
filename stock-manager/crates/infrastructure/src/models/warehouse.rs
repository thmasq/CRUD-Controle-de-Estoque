use chrono::{DateTime, Utc};
use diesel::prelude::*;
use stock_domain::entities::warehouse::Warehouse;
use uuid::Uuid;

use crate::schema::warehouses;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = warehouses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WarehouseModel {
	pub id: Uuid,
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = warehouses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWarehouseModel {
	pub id: Uuid,
	pub name: String,
	pub location: String,
	pub contact_info: Option<String>,
	pub is_active: bool,
}

impl From<WarehouseModel> for Warehouse {
	fn from(model: WarehouseModel) -> Self {
		Self {
			id: model.id,
			name: model.name,
			location: model.location,
			contact_info: model.contact_info,
			is_active: model.is_active,
			created_at: model.created_at,
			updated_at: model.updated_at,
		}
	}
}

impl From<Warehouse> for NewWarehouseModel {
	fn from(entity: Warehouse) -> Self {
		Self {
			id: entity.id,
			name: entity.name,
			location: entity.location,
			contact_info: entity.contact_info,
			is_active: entity.is_active,
		}
	}
}

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use stock_domain::entities::product::Product;
use uuid::Uuid;

use crate::schema::products;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductModel {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProductModel {
	pub id: Uuid,
	pub name: String,
	pub description: Option<String>,
	pub sku: String,
	pub category_id: Option<Uuid>,
	pub is_active: bool,
}

impl From<ProductModel> for Product {
	fn from(model: ProductModel) -> Self {
		Self {
			id: model.id,
			name: model.name,
			description: model.description,
			sku: model.sku,
			category_id: model.category_id,
			is_active: model.is_active,
			created_at: model.created_at,
			updated_at: model.updated_at,
		}
	}
}

impl From<Product> for NewProductModel {
	fn from(entity: Product) -> Self {
		Self {
			id: entity.id,
			name: entity.name,
			description: entity.description,
			sku: entity.sku,
			category_id: entity.category_id,
			is_active: entity.is_active,
		}
	}
}

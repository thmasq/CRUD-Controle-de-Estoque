use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use stock_domain::entities::warehouse::Warehouse;
use stock_domain::repositories::warehouse_repository::WarehouseRepository;
use uuid::Uuid;

use crate::db::PgPool;
use crate::models::warehouse::{NewWarehouseModel, WarehouseModel};
use crate::schema::warehouses;

pub struct DieselWarehouseRepository {
    pool: Arc<PgPool>,
}

impl DieselWarehouseRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WarehouseRepository for DieselWarehouseRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Warehouse>> {
        let conn = &mut self.pool.get()?;

        let result = warehouses::table
            .find(id)
            .select(WarehouseModel::as_select())
            .first(conn)
            .optional()?;

        Ok(result.map(Into::into))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Warehouse>> {
        let conn = &mut self.pool.get()?;

        let result = warehouses::table
            .select(WarehouseModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn create(&self, warehouse: Warehouse) -> anyhow::Result<Warehouse> {
        let conn = &mut self.pool.get()?;

        let new_warehouse = NewWarehouseModel::from(warehouse);

        diesel::insert_into(warehouses::table)
            .values(&new_warehouse)
            .execute(conn)?;

        let inserted_warehouse = warehouses::table
            .find(new_warehouse.id)
            .select(WarehouseModel::as_select())
            .first(conn)?;

        Ok(inserted_warehouse.into())
    }

    async fn update(&self, warehouse: Warehouse) -> anyhow::Result<Warehouse> {
        let conn = &mut self.pool.get()?;

        let updated_rows = diesel::update(warehouses::table.find(warehouse.id))
            .set((
                warehouses::name.eq(warehouse.name.clone()),
                warehouses::location.eq(warehouse.location.clone()),
                warehouses::contact_info.eq(warehouse.contact_info.clone()),
                warehouses::is_active.eq(warehouse.is_active),
                warehouses::updated_at.eq(Utc::now()),
            ))
            .execute(conn)?;

        if updated_rows == 0 {
            return Err(anyhow::anyhow!("Warehouse not found"));
        }

        let updated_warehouse = warehouses::table
            .find(warehouse.id)
            .select(WarehouseModel::as_select())
            .first(conn)?;

        Ok(updated_warehouse.into())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<bool> {
        let conn = &mut self.pool.get()?;

        let deleted_rows = diesel::delete(warehouses::table.find(id)).execute(conn)?;

        Ok(deleted_rows > 0)
    }
}

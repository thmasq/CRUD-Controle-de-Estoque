use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use stock_domain::entities::stock_item::StockItem;
use stock_domain::repositories::stock_item_repository::StockItemRepository;
use uuid::Uuid;

use crate::db::PgPool;
use crate::models::stock_item::{NewStockItemModel, StockItemModel};
use crate::schema::stock_items;

pub struct DieselStockItemRepository {
    pool: Arc<PgPool>,
}

impl DieselStockItemRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StockItemRepository for DieselStockItemRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<StockItem>> {
        let conn = &mut self.pool.get()?;

        let result = stock_items::table
            .find(id)
            .select(StockItemModel::as_select())
            .first(conn)
            .optional()?;

        Ok(result.map(Into::into))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<StockItem>> {
        let conn = &mut self.pool.get()?;

        let result = stock_items::table
            .select(StockItemModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn find_by_product(&self, product_id: Uuid) -> anyhow::Result<Vec<StockItem>> {
        let conn = &mut self.pool.get()?;

        let result = stock_items::table
            .filter(stock_items::product_id.eq(product_id))
            .select(StockItemModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn find_by_warehouse(&self, warehouse_id: Uuid) -> anyhow::Result<Vec<StockItem>> {
        let conn = &mut self.pool.get()?;

        let result = stock_items::table
            .filter(stock_items::warehouse_id.eq(warehouse_id))
            .select(StockItemModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn find_by_product_and_warehouse(
        &self,
        product_id: Uuid,
        warehouse_id: Uuid,
    ) -> anyhow::Result<Option<StockItem>> {
        let conn = &mut self.pool.get()?;

        let result = stock_items::table
            .filter(stock_items::product_id.eq(product_id))
            .filter(stock_items::warehouse_id.eq(warehouse_id))
            .select(StockItemModel::as_select())
            .first(conn)
            .optional()?;

        Ok(result.map(Into::into))
    }

    async fn create(&self, stock_item: StockItem) -> anyhow::Result<StockItem> {
        let conn = &mut self.pool.get()?;

        let new_stock_item = NewStockItemModel::from(stock_item);

        diesel::insert_into(stock_items::table)
            .values(&new_stock_item)
            .execute(conn)?;

        let inserted_stock_item = stock_items::table
            .find(new_stock_item.id)
            .select(StockItemModel::as_select())
            .first(conn)?;

        Ok(inserted_stock_item.into())
    }

    async fn update(&self, stock_item: StockItem) -> anyhow::Result<StockItem> {
        let conn = &mut self.pool.get()?;

        let updated_rows = diesel::update(stock_items::table.find(stock_item.id))
            .set((
                stock_items::quantity.eq(stock_item.quantity),
                stock_items::unit_cost.eq(stock_item.unit_cost),
                stock_items::last_restocked.eq(stock_item.last_restocked),
                stock_items::updated_at.eq(Utc::now()),
            ))
            .execute(conn)?;

        if updated_rows == 0 {
            return Err(anyhow::anyhow!("Stock item not found"));
        }

        let updated_stock_item = stock_items::table
            .find(stock_item.id)
            .select(StockItemModel::as_select())
            .first(conn)?;

        Ok(updated_stock_item.into())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<bool> {
        let conn = &mut self.pool.get()?;

        let deleted_rows = diesel::delete(stock_items::table.find(id)).execute(conn)?;

        Ok(deleted_rows > 0)
    }
}

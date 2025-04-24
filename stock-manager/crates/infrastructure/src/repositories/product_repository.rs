use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use stock_domain::entities::product::Product;
use stock_domain::repositories::product_repository::ProductRepository;
use uuid::Uuid;

use crate::db::PgPool;
use crate::models::product::{NewProductModel, ProductModel};
use crate::schema::products;

pub struct DieselProductRepository {
    pool: Arc<PgPool>,
}

impl DieselProductRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for DieselProductRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Product>> {
        let conn = &mut self.pool.get()?;

        let result = products::table
            .find(id)
            .select(ProductModel::as_select())
            .first(conn)
            .optional()?;

        Ok(result.map(Into::into))
    }

    async fn find_by_sku(&self, sku: &str) -> anyhow::Result<Option<Product>> {
        let conn = &mut self.pool.get()?;

        let result = products::table
            .filter(products::sku.eq(sku))
            .select(ProductModel::as_select())
            .first(conn)
            .optional()?;

        Ok(result.map(Into::into))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Product>> {
        let conn = &mut self.pool.get()?;

        let result = products::table
            .select(ProductModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn find_by_category(&self, category_id: Uuid) -> anyhow::Result<Vec<Product>> {
        let conn = &mut self.pool.get()?;

        let result = products::table
            .filter(products::category_id.eq(category_id))
            .select(ProductModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn create(&self, product: Product) -> anyhow::Result<Product> {
        let conn = &mut self.pool.get()?;

        let new_product = NewProductModel::from(product);

        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(conn)?;

        let inserted_product = products::table
            .find(new_product.id)
            .select(ProductModel::as_select())
            .first(conn)?;

        Ok(inserted_product.into())
    }

    async fn update(&self, product: Product) -> anyhow::Result<Product> {
        let conn = &mut self.pool.get()?;

        let updated_rows = diesel::update(products::table.find(product.id))
            .set((
                products::name.eq(product.name.clone()),
                products::description.eq(product.description.clone()),
                products::sku.eq(product.sku.clone()),
                products::category_id.eq(product.category_id),
                products::is_active.eq(product.is_active),
                products::updated_at.eq(Utc::now()),
            ))
            .execute(conn)?;

        if updated_rows == 0 {
            return Err(anyhow::anyhow!("Product not found"));
        }

        let updated_product = products::table
            .find(product.id)
            .select(ProductModel::as_select())
            .first(conn)?;

        Ok(updated_product.into())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<bool> {
        let conn = &mut self.pool.get()?;

        let deleted_rows = diesel::delete(products::table.find(id)).execute(conn)?;

        Ok(deleted_rows > 0)
    }
}

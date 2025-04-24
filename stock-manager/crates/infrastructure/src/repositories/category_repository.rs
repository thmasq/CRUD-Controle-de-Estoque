use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;
use stock_domain::entities::category::Category;
use stock_domain::repositories::category_repository::CategoryRepository;
use uuid::Uuid;

use crate::db::PgPool;
use crate::models::category::{CategoryModel, NewCategoryModel};
use crate::schema::categories;

pub struct DieselCategoryRepository {
    pool: Arc<PgPool>,
}

impl DieselCategoryRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for DieselCategoryRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        let conn = &mut self.pool.get()?;

        let result = categories::table
            .find(id)
            .select(CategoryModel::as_select())
            .first(conn)
            .optional()?;

        Ok(result.map(Into::into))
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Category>> {
        let conn = &mut self.pool.get()?;

        let result = categories::table
            .select(CategoryModel::as_select())
            .load(conn)?;

        Ok(result.into_iter().map(Into::into).collect())
    }

    async fn create(&self, category: Category) -> anyhow::Result<Category> {
        let conn = &mut self.pool.get()?;

        let new_category = NewCategoryModel::from(category);

        diesel::insert_into(categories::table)
            .values(&new_category)
            .execute(conn)?;

        let inserted_category = categories::table
            .find(new_category.id)
            .select(CategoryModel::as_select())
            .first(conn)?;

        Ok(inserted_category.into())
    }

    async fn update(&self, category: Category) -> anyhow::Result<Category> {
        let conn = &mut self.pool.get()?;

        let updated_rows = diesel::update(categories::table.find(category.id))
            .set((
                categories::name.eq(category.name.clone()),
                categories::description.eq(category.description.clone()),
                categories::updated_at.eq(Utc::now()),
            ))
            .execute(conn)?;

        if updated_rows == 0 {
            return Err(anyhow::anyhow!("Category not found"));
        }

        let updated_category = categories::table
            .find(category.id)
            .select(CategoryModel::as_select())
            .first(conn)?;

        Ok(updated_category.into())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<bool> {
        let conn = &mut self.pool.get()?;

        let deleted_rows = diesel::delete(categories::table.find(id)).execute(conn)?;

        Ok(deleted_rows > 0)
    }
}

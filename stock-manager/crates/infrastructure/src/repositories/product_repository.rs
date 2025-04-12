use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use stock_domain::errors::{DomainError, DomainResult};
use stock_domain::models::Product;
use stock_domain::repositories::ProductRepository;

pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Product> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, sku, created_at, updated_at
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        product.ok_or_else(|| DomainError::NotFound(format!("Product with id {} not found", id)))
    }

    async fn find_all(&self) -> DomainResult<Vec<Product>> {
        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT id, name, description, sku, created_at, updated_at
            FROM products
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(products)
    }

    async fn save(&self, product: Product) -> DomainResult<Product> {
        let updated_product = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (id, name, description, sku, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
            SET name = $2, description = $3, sku = $4, updated_at = $6
            RETURNING id, name, description, sku, created_at, updated_at
            "#,
            product.id,
            product.name,
            product.description,
            product.sku,
            product.created_at,
            product.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(updated_product)
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM products
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DomainError::NotFound(format!(
                "Product with id {} not found",
                id
            )));
        }

        Ok(())
    }
}

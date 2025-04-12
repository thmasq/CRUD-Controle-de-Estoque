use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use stock_domain::errors::{DomainError, DomainResult};
use stock_domain::models::StockItem;
use stock_domain::repositories::StockRepository;

pub struct PostgresStockRepository {
    pool: PgPool,
}

impl PostgresStockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StockRepository for PostgresStockRepository {
    async fn find_by_product_id(&self, product_id: Uuid) -> DomainResult<Vec<StockItem>> {
        let stock_items = sqlx::query_as!(
            StockItem,
            r#"
            SELECT id, product_id, quantity, location, unit_cost, last_restocked
            FROM stock_items
            WHERE product_id = $1
            "#,
            product_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(stock_items)
    }

    async fn find_by_id(&self, id: Uuid) -> DomainResult<StockItem> {
        let stock_item = sqlx::query_as!(
            StockItem,
            r#"
            SELECT id, product_id, quantity, location, unit_cost, last_restocked
            FROM stock_items
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        stock_item
            .ok_or_else(|| DomainError::NotFound(format!("StockItem with id {} not found", id)))
    }

    async fn update_quantity(&self, id: Uuid, quantity: i32) -> DomainResult<StockItem> {
        if quantity < 0 {
            return Err(DomainError::InvalidOperation(
                "Quantity cannot be negative".to_string(),
            ));
        }

        let now = chrono::Utc::now();
        let stock_item = sqlx::query_as!(
            StockItem,
            r#"
            UPDATE stock_items
            SET quantity = $2, last_restocked = $3
            WHERE id = $1
            RETURNING id, product_id, quantity, location, unit_cost, last_restocked
            "#,
            id,
            quantity,
            now
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        stock_item
            .ok_or_else(|| DomainError::NotFound(format!("StockItem with id {} not found", id)))
    }

    async fn save(&self, stock_item: StockItem) -> DomainResult<StockItem> {
        let updated_stock_item = sqlx::query_as!(
            StockItem,
            r#"
            INSERT INTO stock_items (id, product_id, quantity, location, unit_cost, last_restocked)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
            SET product_id = $2, quantity = $3, location = $4, unit_cost = $5, last_restocked = $6
            RETURNING id, product_id, quantity, location, unit_cost, last_restocked
            "#,
            stock_item.id,
            stock_item.product_id,
            stock_item.quantity,
            stock_item.location,
            stock_item.unit_cost,
            stock_item.last_restocked
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(updated_stock_item)
    }
}

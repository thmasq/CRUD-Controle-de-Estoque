use async_trait::async_trait;
use uuid::Uuid;

use crate::errors::DomainResult;
use crate::models::{Product, StockItem};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Product>;
    async fn find_all(&self) -> DomainResult<Vec<Product>>;
    async fn save(&self, product: Product) -> DomainResult<Product>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}

#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn find_by_product_id(&self, product_id: Uuid) -> DomainResult<Vec<StockItem>>;
    async fn find_by_id(&self, id: Uuid) -> DomainResult<StockItem>;
    async fn update_quantity(&self, id: Uuid, quantity: i32) -> DomainResult<StockItem>;
    async fn save(&self, stock_item: StockItem) -> DomainResult<StockItem>;
}

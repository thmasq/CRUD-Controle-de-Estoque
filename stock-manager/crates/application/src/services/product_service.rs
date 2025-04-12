use bigdecimal::BigDecimal;
use std::sync::Arc;
use uuid::Uuid;

use stock_domain::{
    DomainError, DomainResult, Product, ProductRepository, StockItem, StockRepository,
};

pub struct ProductService<P, S> {
    product_repo: Arc<P>,
    pub stock_repo: Arc<S>,
}

impl<P, S> ProductService<P, S>
where
    P: ProductRepository,
    S: StockRepository,
{
    pub fn new(product_repo: Arc<P>, stock_repo: Arc<S>) -> Self {
        Self {
            product_repo,
            stock_repo,
        }
    }

    pub async fn get_product(&self, id: Uuid) -> DomainResult<Product> {
        self.product_repo.find_by_id(id).await
    }

    pub async fn get_all_products(&self) -> DomainResult<Vec<Product>> {
        self.product_repo.find_all().await
    }

    pub async fn create_product(
        &self,
        name: String,
        description: Option<String>,
        sku: String,
    ) -> DomainResult<Product> {
        let product = Product::new(name, description, sku);
        self.product_repo.save(product).await
    }

    pub async fn update_product(
        &self,
        id: Uuid,
        name: String,
        description: Option<String>,
        sku: String,
    ) -> DomainResult<Product> {
        let mut product = self.product_repo.find_by_id(id).await?;
        product.update(name, description, sku);
        self.product_repo.save(product).await
    }

    pub async fn delete_product(&self, id: Uuid) -> DomainResult<()> {
        // Check if there are any stock items for this product
        let stock_items = self.stock_repo.find_by_product_id(id).await?;
        if !stock_items.is_empty() {
            return Err(DomainError::InvalidOperation(
                "Cannot delete product with existing stock items".to_string(),
            ));
        }

        self.product_repo.delete(id).await
    }

    pub async fn get_product_stock(&self, product_id: Uuid) -> DomainResult<Vec<StockItem>> {
        // First check if the product exists
        self.product_repo.find_by_id(product_id).await?;
        // Then get the stock items
        self.stock_repo.find_by_product_id(product_id).await
    }

    pub async fn add_stock_item(
        &self,
        product_id: Uuid,
        quantity: i32,
        location: String,
        unit_cost: BigDecimal,
    ) -> DomainResult<StockItem> {
        // Verify the product exists
        self.product_repo.find_by_id(product_id).await?;

        if quantity < 0 {
            return Err(DomainError::InvalidOperation(
                "Quantity cannot be negative".to_string(),
            ));
        }

        let stock_item = StockItem::new(product_id, quantity, location, unit_cost);
        self.stock_repo.save(stock_item).await
    }

    pub async fn update_stock_quantity(&self, id: Uuid, quantity: i32) -> DomainResult<StockItem> {
        if quantity < 0 {
            return Err(DomainError::InvalidOperation(
                "Quantity cannot be negative".to_string(),
            ));
        }

        self.stock_repo.update_quantity(id, quantity).await
    }
}

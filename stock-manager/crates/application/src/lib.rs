pub mod services;

// Re-export services for easier access
pub use services::category_service::{CategoryCreateDto, CategoryService, CategoryUpdateDto};
pub use services::product_service::{ProductCreateDto, ProductService, ProductUpdateDto};
pub use services::stock_item_service::{StockItemCreateDto, StockItemService, StockItemUpdateDto};
pub use services::stock_transaction_service::{StockTransactionCreateDto, StockTransactionService};
pub use services::warehouse_service::{WarehouseCreateDto, WarehouseService, WarehouseUpdateDto};

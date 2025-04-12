pub mod db;
pub mod repositories;

// Re-export for convenience
pub use db::create_pool;
pub use repositories::{PostgresProductRepository, PostgresStockRepository};

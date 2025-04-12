pub mod errors;
pub mod models;
pub mod repositories;

// Re-export for convenience
pub use errors::{DomainError, DomainResult};
pub use models::{Product, StockItem};
pub use repositories::{ProductRepository, StockRepository};

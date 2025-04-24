pub mod db;
pub mod models;
pub mod repositories;
pub mod schema;

// Re-export for easier access
pub use db::{PgPool, PgPooledConnection, establish_connection_pool};

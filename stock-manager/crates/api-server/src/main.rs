// stock-manager/crates/api-server/src/main.rs
use std::sync::Arc;

use actix_web::{App, HttpServer, middleware, web};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod dtos;
mod handlers;

use handlers::{
    create_product, create_stock_item, delete_product, get_product, get_product_stock,
    get_products, update_product, update_stock_quantity,
};
use stock_application::ProductService;
use stock_infrastructure::{PostgresProductRepository, PostgresStockRepository, create_pool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/stock_manager".to_string());

    // Create DB pool
    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create DB pool");

    // Create repositories
    let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
    let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));

    // Create service
    let product_service = Arc::new(ProductService::new(product_repo, stock_repo));

    // Start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::from(product_service.clone()))
            .service(
                web::scope("/api")
                    .service(get_products)
                    .service(get_product)
                    .service(create_product)
                    .service(update_product)
                    .service(delete_product)
                    .service(get_product_stock)
                    .service(create_stock_item)
                    .service(update_stock_quantity),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    println!("Server running at http://0.0.0.0:8080");

    server.await
}

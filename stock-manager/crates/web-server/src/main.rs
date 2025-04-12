use std::sync::Arc;

use actix_files as fs;
use actix_web::{App, HttpServer, middleware, web};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

use handlers::{
    create_product, create_stock_item, delete_product, index, products_index, stock_index,
    update_quantity, update_quantity_form,
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
            .service(fs::Files::new("/static", "./static"))
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            )
            .service(
                web::resource("/products")
                    .route(web::get().to(products_index::<PostgresProductRepository, PostgresStockRepository>))
                    .route(web::post().to(create_product::<PostgresProductRepository, PostgresStockRepository>))
            )
            .service(
                web::resource("/products/{id}")
                    .route(web::delete().to(delete_product::<PostgresProductRepository, PostgresStockRepository>))
            )
            .service(
                web::resource("/stock")
                    .route(web::get().to(stock_index::<PostgresProductRepository, PostgresStockRepository>))
                    .route(web::post().to(create_stock_item::<PostgresProductRepository, PostgresStockRepository>))
            )
            .service(
                web::resource("/stock/{id}/update-quantity")
                    .route(web::get().to(update_quantity_form::<PostgresProductRepository, PostgresStockRepository>))
            )
            .service(
                web::resource("/stock/{id}/quantity")
                    .route(web::put().to(update_quantity::<PostgresProductRepository, PostgresStockRepository>))
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run();

    println!("Web server running at http://0.0.0.0:8081");

    server.await
}

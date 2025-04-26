use std::env;
use std::sync::Arc;

use actix_web::{App, HttpServer, middleware, web};
use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod dtos;
mod filters;
mod handlers;
mod static_assets;

use stock_application::{CategoryService, ProductService, StockItemService, StockTransactionService, WarehouseService};
use stock_infrastructure::db::establish_connection_pool;
use stock_infrastructure::repositories::category_repository::DieselCategoryRepository;
use stock_infrastructure::repositories::product_repository::DieselProductRepository;
use stock_infrastructure::repositories::stock_item_repository::DieselStockItemRepository;
use stock_infrastructure::repositories::stock_transaction_repository::DieselStockTransactionRepository;
use stock_infrastructure::repositories::warehouse_repository::DieselWarehouseRepository;

// Application state that holds all services
pub struct AppState {
	category_service: Arc<CategoryService>,
	product_service: Arc<ProductService>,
	warehouse_service: Arc<WarehouseService>,
	stock_item_service: Arc<StockItemService>,
	transaction_service: Arc<StockTransactionService>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	// Initialize tracing
	tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::new(
			std::env::var("RUST_LOG").unwrap_or_else(|_| "stock_web_server=debug,actix_web=info".into()),
		))
		.with(tracing_subscriber::fmt::layer())
		.init();

	// Create DB connection pool
	let pool = establish_connection_pool();
	let pool = Arc::new(pool);

	// Create repositories
	let category_repo = Arc::new(DieselCategoryRepository::new(pool.clone()));
	let product_repo = Arc::new(DieselProductRepository::new(pool.clone()));
	let warehouse_repo = Arc::new(DieselWarehouseRepository::new(pool.clone()));
	let stock_item_repo = Arc::new(DieselStockItemRepository::new(pool.clone()));
	let transaction_repo = Arc::new(DieselStockTransactionRepository::new(pool.clone()));

	// Create services
	let category_service = Arc::new(CategoryService::new(category_repo));
	let product_service = Arc::new(ProductService::new(product_repo));
	let warehouse_service = Arc::new(WarehouseService::new(warehouse_repo));
	let stock_item_service = Arc::new(StockItemService::new(stock_item_repo.clone()));
	let transaction_service = Arc::new(StockTransactionService::new(transaction_repo, stock_item_repo));

	// Create app state
	let app_state = web::Data::new(AppState {
		category_service,
		product_service,
		warehouse_service,
		stock_item_service,
		transaction_service,
	});

	let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
	let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
	let bind_address = format!("{host}:{port}");

	// Start HTTP server
	let server = HttpServer::new(move || {
		App::new()
			.wrap(middleware::Logger::default())
			.app_data(app_state.clone())
			.configure(static_assets::register)
			// Dashboard
			.route("/", web::get().to(handlers::dashboard::index))
			// Categories
			.route("/categories", web::get().to(handlers::category::list_categories))
			.route("/categories/new", web::get().to(handlers::category::new_category_form))
			.route(
				"/categories/{id}/edit",
				web::get().to(handlers::category::edit_category_form),
			)
			.route("/categories", web::post().to(handlers::category::create_category))
			.route("/categories/{id}", web::put().to(handlers::category::update_category))
			.route(
				"/categories/{id}",
				web::delete().to(handlers::category::delete_category),
			)
			// Products
			.route("/products", web::get().to(handlers::product::list_products))
			.route("/products/new", web::get().to(handlers::product::new_product_form))
			.route(
				"/products/{id}/edit",
				web::get().to(handlers::product::edit_product_form),
			)
			.route("/products", web::post().to(handlers::product::create_product))
			.route("/products/{id}", web::put().to(handlers::product::update_product))
			.route("/products/{id}", web::delete().to(handlers::product::delete_product))
			// Warehouses
			.route("/warehouses", web::get().to(handlers::warehouse::list_warehouses))
			.route(
				"/warehouses/new",
				web::get().to(handlers::warehouse::new_warehouse_form),
			)
			.route(
				"/warehouses/{id}/edit",
				web::get().to(handlers::warehouse::edit_warehouse_form),
			)
			.route("/warehouses", web::post().to(handlers::warehouse::create_warehouse))
			.route("/warehouses/{id}", web::put().to(handlers::warehouse::update_warehouse))
			.route(
				"/warehouses/{id}",
				web::delete().to(handlers::warehouse::delete_warehouse),
			)
			// Stock Items
			.route("/stock-items", web::get().to(handlers::stock_item::list_stock_items))
			.route(
				"/stock-items/new",
				web::get().to(handlers::stock_item::new_stock_item_form),
			)
			.route(
				"/stock-items/{id}/edit",
				web::get().to(handlers::stock_item::edit_stock_item_form),
			)
			.route(
				"/stock-items/{id}/transaction",
				web::get().to(handlers::stock_item::transaction_form),
			)
			.route("/stock-items", web::post().to(handlers::stock_item::create_stock_item))
			.route(
				"/stock-items/{id}",
				web::put().to(handlers::stock_item::update_stock_item),
			)
			.route(
				"/stock-items/{id}",
				web::delete().to(handlers::stock_item::delete_stock_item),
			)
			.route(
				"/stock-items/{id}/transaction",
				web::post().to(handlers::stock_item::create_transaction),
			)
			// Transactions
			.route(
				"/transactions",
				web::get().to(handlers::stock_transaction::list_transactions),
			)
	})
	.bind(&bind_address)?
	.run();

	println!("Server running at http://{bind_address}");
	server.await
}

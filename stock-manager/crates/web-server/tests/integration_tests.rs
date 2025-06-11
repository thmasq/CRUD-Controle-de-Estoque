use actix_web::http::StatusCode;
use actix_web::{App, test, web};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use stock_application::services::auth_service::AuthService;
use stock_application::{CategoryService, ProductService, StockItemService, StockTransactionService, WarehouseService};
use stock_infrastructure::db::establish_connection_pool;
use stock_infrastructure::repositories::category_repository::DieselCategoryRepository;
use stock_infrastructure::repositories::product_repository::DieselProductRepository;
use stock_infrastructure::repositories::stock_item_repository::DieselStockItemRepository;
use stock_infrastructure::repositories::stock_transaction_repository::DieselStockTransactionRepository;
use stock_infrastructure::repositories::user_repository::DieselUserRepository;
use stock_infrastructure::repositories::warehouse_repository::DieselWarehouseRepository;
use stock_web_server::{AppState, handlers, middleware};

struct TestContext {
	auth_token: Option<String>,
	username: Option<String>,
	app_state: web::Data<AppState>,
}

impl TestContext {
	async fn new() -> Self {
		// Set up database connection
		let pool = establish_connection_pool();
		let pool = Arc::new(pool);

		// Create repositories
		let category_repo = Arc::new(DieselCategoryRepository::new(pool.clone()));
		let product_repo = Arc::new(DieselProductRepository::new(pool.clone()));
		let warehouse_repo = Arc::new(DieselWarehouseRepository::new(pool.clone()));
		let stock_item_repo = Arc::new(DieselStockItemRepository::new(pool.clone()));
		let transaction_repo = Arc::new(DieselStockTransactionRepository::new(pool.clone()));
		let user_repo = Arc::new(DieselUserRepository::new(pool.clone()));

		// Create services
		let category_service = Arc::new(CategoryService::new(category_repo));
		let product_service = Arc::new(ProductService::new(product_repo));
		let warehouse_service = Arc::new(WarehouseService::new(warehouse_repo));
		let stock_item_service = Arc::new(StockItemService::new(stock_item_repo.clone()));
		let transaction_service = Arc::new(StockTransactionService::new(transaction_repo, stock_item_repo));
		let auth_service = Arc::new(AuthService::new(user_repo, "test_secret".to_string()));

		let app_state = web::Data::new(AppState {
			category_service,
			product_service,
			warehouse_service,
			stock_item_service,
			transaction_service,
			auth_service,
			jwt_secret: "test_secret".to_string(),
			enable_registration: true,
		});

		Self {
			auth_token: None,
			username: None,
			app_state,
		}
	}

	fn create_test_app(
		&self,
	) -> actix_web::App<
		impl actix_web::dev::ServiceFactory<
			actix_web::dev::ServiceRequest,
			Config = (),
			Response = actix_web::dev::ServiceResponse,
			Error = actix_web::Error,
			InitError = (),
		> + use<>,
	> {
		let mut app = App::new()
			.wrap(middleware::auth::Authentication {
				exclude_paths: vec![
					"/auth/login".to_string(),
					"/auth/register".to_string(),
					"/_static".to_string(),
				],
			})
			.app_data(self.app_state.clone())
			.configure(stock_web_server::configure_routes);

		// Add registration routes since tests need them
		if self.app_state.enable_registration {
			app = app
				.route("/auth/register", web::get().to(handlers::auth::register_form))
				.route("/auth/register", web::post().to(handlers::auth::register));
		}

		app
	}

	async fn register_and_login(
		&mut self,
		username: &str,
		password: &str,
		role: &str,
	) -> Result<(), Box<dyn std::error::Error>> {
		let app = test::init_service(self.create_test_app()).await;

		// Register user
		let register_data = json!({
			"username": username,
			"password": password,
			"role": role
		});

		let req = test::TestRequest::post()
			.uri("/auth/register")
			.insert_header(("content-type", "application/x-www-form-urlencoded"))
			.set_form(&register_data)
			.to_request();

		let resp = test::call_service(&app, req).await;
		assert!(resp.status().is_success() || resp.status() == StatusCode::FOUND);

		// Login
		let login_data = json!({
			"username": username,
			"password": password
		});

		let req = test::TestRequest::post()
			.uri("/auth/login")
			.insert_header(("content-type", "application/x-www-form-urlencoded"))
			.set_form(&login_data)
			.to_request();

		let resp = test::call_service(&app, req).await;
		assert!(resp.status().is_success() || resp.status() == StatusCode::FOUND);

		// Extract auth token from cookies
		if let Some(set_cookie_header) = resp.headers().get("set-cookie") {
			let cookie_str = set_cookie_header.to_str()?;
			if cookie_str.contains("auth_token=") {
				let token_start = cookie_str.find("auth_token=").unwrap() + "auth_token=".len();
				let token_end = cookie_str[token_start..]
					.find(';')
					.unwrap_or(cookie_str.len() - token_start);
				let token = &cookie_str[token_start..token_start + token_end];
				self.auth_token = Some(token.to_string());
				self.username = Some(username.to_string());
			}
		}

		Ok(())
	}

	fn add_auth_cookie(&self, req: test::TestRequest) -> test::TestRequest {
		if let Some(token) = &self.auth_token {
			req.cookie(actix_web::cookie::Cookie::new("auth_token", token))
		} else {
			req
		}
	}
}

#[actix_web::test]
async fn test_auth_flow() {
	let mut ctx = TestContext::new().await;
	let app = test::init_service(ctx.create_test_app()).await;

	// Test registration page loads
	let req = test::TestRequest::get().uri("/auth/register").to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test login page loads
	let req = test::TestRequest::get().uri("/auth/login").to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test successful registration and login
	ctx.register_and_login("testuser", "testpass", "MANAGER").await.unwrap();
	assert!(ctx.auth_token.is_some());
}

#[actix_web::test]
async fn test_dashboard_requires_auth() {
	let ctx = TestContext::new().await;
	let app = test::init_service(ctx.create_test_app()).await;

	// Test that dashboard redirects when not authenticated
	let req = test::TestRequest::get().uri("/").to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_dashboard_with_auth() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("dashuser", "testpass", "MANAGER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test that dashboard loads when authenticated
	let req = ctx.add_auth_cookie(test::TestRequest::get().uri("/")).to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_category_crud() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("catuser", "testpass", "MANAGER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test list categories (empty initially)
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/categories"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test create category
	let category_data = json!({
		"name": "Electronics",
		"description": "Electronic products"
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/categories"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&category_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// Test get category form
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/categories/new"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_product_crud() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("produser", "testpass", "MANAGER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test list products
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test create product
	let product_data = json!({
		"name": "Test Product",
		"description": "A test product",
		"sku": "TEST-001",
		"is_active": true
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/products"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&product_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// Test get product form
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products/new"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_warehouse_crud() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("warehouseuser", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test list warehouses
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/warehouses"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test create warehouse
	let warehouse_data = json!({
		"name": "Main Warehouse",
		"location": "123 Main St",
		"contact_info": "contact@example.com",
		"is_active": true
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/warehouses"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&warehouse_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// Test get warehouse form
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/warehouses/new"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_stock_items_crud() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("stockuser", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test list stock items
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/stock-items"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test get stock item form
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/stock-items/new"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_transactions_list() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("transuser", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test list transactions
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/transactions"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_filtering() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("filteruser", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test product filtering
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products?status=active"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test transaction filtering
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/transactions?transaction_type=IN"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_htmx_requests() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("htmxuser", "testpass", "MANAGER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test HTMX product list request
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products"))
		.insert_header(("HX-Request", "true"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test HTMX transaction list request
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/transactions"))
		.insert_header(("HX-Request", "true"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_unauthorized_access() {
	let ctx = TestContext::new().await;
	let app = test::init_service(ctx.create_test_app()).await;

	// Test that protected endpoints require authentication
	let protected_endpoints = vec![
		"/categories",
		"/products",
		"/warehouses",
		"/stock-items",
		"/transactions",
	];

	for endpoint in protected_endpoints {
		let req = test::TestRequest::get().uri(endpoint).to_request();
		let resp = test::call_service(&app, req).await;
		assert_eq!(
			resp.status(),
			StatusCode::UNAUTHORIZED,
			"Endpoint {} should require auth",
			endpoint
		);
	}
}

#[actix_web::test]
async fn test_static_assets() {
	let ctx = TestContext::new().await;
	let app = test::init_service(ctx.create_test_app()).await;

	// Test that static assets are accessible without auth
	let req = test::TestRequest::get().uri("/_static/htmx.min.js").to_request();
	let resp = test::call_service(&app, req).await;
	// This might be 404 if the file doesn't exist in tests, but should not be unauthorized
	assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_error_handling() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("erroruser", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test accessing non-existent category
	let fake_id = Uuid::new_v4();
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri(&format!("/categories/{}/edit", fake_id)))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);

	// Test accessing non-existent product
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri(&format!("/products/{}/edit", fake_id)))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_duplicate_creation() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("dupuser", "testpass", "MANAGER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Create a product with a specific SKU
	let product_data = json!({
		"name": "Unique Product",
		"sku": "UNIQUE-001",
		"is_active": true
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/products"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&product_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// Try to create another product with the same SKU
	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/products"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&product_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn test_complete_workflow() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("workflowuser", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// 1. Create a category
	let category_data = json!({
		"name": "Test Category",
		"description": "Category for testing"
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/categories"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&category_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// 2. Create a warehouse
	let warehouse_data = json!({
		"name": "Test Warehouse",
		"location": "Test Location",
		"is_active": true
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/warehouses"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&warehouse_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// 3. Create a product
	let product_data = json!({
		"name": "Test Product",
		"sku": "WORKFLOW-001",
		"is_active": true
	});

	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/products"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&product_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// 4. Verify all lists work
	let endpoints = vec![
		"/categories",
		"/warehouses",
		"/products",
		"/stock-items",
		"/transactions",
	];
	for endpoint in endpoints {
		let req = ctx.add_auth_cookie(test::TestRequest::get().uri(endpoint)).to_request();
		let resp = test::call_service(&app, req).await;
		assert_eq!(resp.status(), StatusCode::OK, "Failed to access {}", endpoint);
	}
}

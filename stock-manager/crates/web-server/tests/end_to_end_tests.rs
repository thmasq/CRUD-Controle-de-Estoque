use actix_web::http::StatusCode;
use actix_web::test;
use serde_json::json;
use std::sync::Arc;

use stock_application::services::auth_service::AuthService;
use stock_application::{CategoryService, ProductService, StockItemService, StockTransactionService, WarehouseService};
use stock_infrastructure::db::establish_connection_pool;
use stock_infrastructure::repositories::category_repository::DieselCategoryRepository;
use stock_infrastructure::repositories::product_repository::DieselProductRepository;
use stock_infrastructure::repositories::stock_item_repository::DieselStockItemRepository;
use stock_infrastructure::repositories::stock_transaction_repository::DieselStockTransactionRepository;
use stock_infrastructure::repositories::user_repository::DieselUserRepository;
use stock_infrastructure::repositories::warehouse_repository::DieselWarehouseRepository;
use stock_web_server::AppState;

mod test_utils;
use test_utils::*;

// Include the TestContext from integration_tests
struct TestContext {
	auth_token: Option<String>,
	username: Option<String>,
	app_state: actix_web::web::Data<AppState>,
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

		let app_state = actix_web::web::Data::new(AppState {
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
		use actix_web::{App, web};

		let mut app = App::new()
			.wrap(stock_web_server::middleware::auth::Authentication {
				exclude_paths: vec![
					"/auth/login".to_string(),
					"/auth/register".to_string(),
					"/_static".to_string(),
					"/".to_string(),
				],
			})
			.app_data(self.app_state.clone())
			.configure(stock_web_server::configure_routes);

		// Add registration routes since tests need them
		if self.app_state.enable_registration {
			app = app
				.route(
					"/auth/register",
					web::get().to(stock_web_server::handlers::auth::register_form),
				)
				.route(
					"/auth/register",
					web::post().to(stock_web_server::handlers::auth::register),
				);
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

fn create_form_data(data: serde_json::Value) -> serde_json::Value {
	if let serde_json::Value::Object(mut map) = data {
		// Remove null values to avoid serialization issues
		map.retain(|_, v| !v.is_null());
		// Convert remaining None values to empty strings if needed
		for (_, v) in map.iter_mut() {
			if v.is_null() {
				*v = serde_json::Value::String("".to_string());
			}
		}
		serde_json::Value::Object(map)
	} else {
		data
	}
}

#[actix_web::test]
async fn test_complete_stock_management_workflow() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("e2euser", "testpass", "MANAGER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;
	let mut test_data = TestData::sample();

	// Step 1: Create categories
	for category in &mut test_data.categories {
		let category_data = create_form_data(json!({
			"name": category.name,
			"description": category.description
		}));

		let req = ctx
			.add_auth_cookie(test::TestRequest::post().uri("/categories"))
			.insert_header(("content-type", "application/x-www-form-urlencoded"))
			.set_form(&category_data)
			.to_request();

		let resp = test::call_service(&app, req).await;
		assert!(
			resp.status().is_success(),
			"Failed to create category: {}",
			category.name
		);
	}

	// Step 2: Create warehouses
	for warehouse in &mut test_data.warehouses {
		let warehouse_data = create_form_data(json!({
			"name": warehouse.name,
			"location": warehouse.location,
			"contact_info": warehouse.contact_info,
			"is_active": warehouse.is_active
		}));

		let req = ctx
			.add_auth_cookie(test::TestRequest::post().uri("/warehouses"))
			.insert_header(("content-type", "application/x-www-form-urlencoded"))
			.set_form(&warehouse_data)
			.to_request();

		let resp = test::call_service(&app, req).await;
		assert!(
			resp.status().is_success(),
			"Failed to create warehouse: {}",
			warehouse.name
		);
	}

	// Step 3: Create products
	for product in &mut test_data.products {
		let product_data = create_form_data(json!({
			"name": product.name,
			"description": product.description,
			"sku": product.sku,
			"is_active": product.is_active
		}));

		let req = ctx
			.add_auth_cookie(test::TestRequest::post().uri("/products"))
			.insert_header(("content-type", "application/x-www-form-urlencoded"))
			.set_form(&product_data)
			.to_request();

		let resp = test::call_service(&app, req).await;
		assert!(resp.status().is_success(), "Failed to create product: {}", product.name);
	}

	// Step 4: Verify all data appears in lists
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/categories"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body = test::read_body(resp).await;
	let body_str = std::str::from_utf8(&body).unwrap();
	assert!(body_str.contains("Electronics"));
	assert!(body_str.contains("Books"));

	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/warehouses"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body = test::read_body(resp).await;
	let body_str = std::str::from_utf8(&body).unwrap();
	assert!(body_str.contains("Main Warehouse"));
	assert!(body_str.contains("Secondary Warehouse"));

	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body = test::read_body(resp).await;
	let body_str = std::str::from_utf8(&body).unwrap();
	assert!(body_str.contains("LAPTOP-001"));
	assert!(body_str.contains("BOOK-RUST-001"));

	// Step 5: Test filtering
	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products?status=active"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Step 6: Test dashboard shows correct counts
	let req = ctx.add_auth_cookie(test::TestRequest::get().uri("/")).to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body = test::read_body(resp).await;
	let body_str = std::str::from_utf8(&body).unwrap();
	// Should show at least the created items
	assert!(body_str.contains("Total Products"));
	assert!(body_str.contains("Total Categories"));
	assert!(body_str.contains("Total Warehouses"));
}

#[actix_web::test]
async fn test_user_roles_and_permissions() {
	// Test with SELLER role
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("seller", "testpass", "SELLER").await.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Sellers should be able to access basic functionality
	let req = ctx.add_auth_cookie(test::TestRequest::get().uri("/")).to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	let req = ctx
		.add_auth_cookie(test::TestRequest::get().uri("/products"))
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::OK);

	// Test with MANAGER role
	let mut ctx2 = TestContext::new().await;
	ctx2.register_and_login("manager", "testpass", "MANAGER").await.unwrap();

	let app2 = test::init_service(ctx2.create_test_app()).await;

	// Managers should have access to all functionality
	let req = ctx2
		.add_auth_cookie(test::TestRequest::get().uri("/categories"))
		.to_request();
	let resp = test::call_service(&app2, req).await;
	assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_error_scenarios() {
	let mut ctx = TestContext::new().await;
	ctx.register_and_login("errortest", "testpass", "MANAGER")
		.await
		.unwrap();

	let app = test::init_service(ctx.create_test_app()).await;

	// Test creating duplicate SKU
	let product_data = json!({
		"name": "Duplicate Test",
		"sku": "DUP-001",
		"is_active": true
	});

	// First creation should succeed
	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/products"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&product_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// Second creation with same SKU should fail
	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/products"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&product_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

	// Test creating category with duplicate name
	let category_data = json!({
		"name": "Duplicate Category"
	});

	// First creation should succeed
	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/categories"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&category_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert!(resp.status().is_success());

	// Second creation with same name should fail
	let req = ctx
		.add_auth_cookie(test::TestRequest::post().uri("/categories"))
		.insert_header(("content-type", "application/x-www-form-urlencoded"))
		.set_form(&category_data)
		.to_request();
	let resp = test::call_service(&app, req).await;
	assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

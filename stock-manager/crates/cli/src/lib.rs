use anyhow::Result;
use dotenv::dotenv;
use std::sync::Arc;

use stock_application::services::auth_service::AuthService;
use stock_infrastructure::db::establish_connection_pool;
use stock_infrastructure::repositories::user_repository::DieselUserRepository;

pub mod commands;

pub use commands::*;

pub struct CliContext {
	pub auth_service: Arc<AuthService>,
}

impl CliContext {
	pub fn new() -> Result<Self> {
		// Load environment variables
		dotenv().ok();

		// Validate DATABASE_URL is set
		if std::env::var("DATABASE_URL").is_err() {
			eprintln!("Error: DATABASE_URL environment variable is not set");
			eprintln!("Please set DATABASE_URL or create a .env file with the database connection string");
			eprintln!("Example: DATABASE_URL=postgres://postgres:postgres@localhost:5432/stockmanager");
			std::process::exit(1);
		}

		// Create database connection pool
		let pool = establish_connection_pool();
		let pool = Arc::new(pool);

		// Create repositories
		let user_repo = Arc::new(DieselUserRepository::new(pool));

		// Create auth service
		let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_cli_secret".to_string());
		let auth_service = Arc::new(AuthService::new(user_repo, jwt_secret));

		Ok(Self { auth_service })
	}
}

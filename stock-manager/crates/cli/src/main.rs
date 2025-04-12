use bigdecimal::BigDecimal;
use clap::{Parser, Subcommand};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use stock_application::ProductService;
use stock_infrastructure::{PostgresProductRepository, PostgresStockRepository, create_pool};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The database URL (defaults to DATABASE_URL environment variable)
    #[arg(short, long)]
    database_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Seed the database with sample data
    Seed,

    /// List all products
    ListProducts,

    /// Add a new product
    AddProduct {
        /// Product name
        #[arg(short, long)]
        name: String,

        /// Product SKU
        #[arg(short, long)]
        sku: String,

        /// Product description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// List stock for a product
    ListStock {
        /// Product ID
        #[arg(short, long)]
        product_id: String,
    },

    /// Add a stock item
    AddStock {
        /// Product ID
        #[arg(short, long)]
        product_id: String,

        /// Location
        #[arg(short, long)]
        location: String,

        /// Quantity
        #[arg(short, long)]
        quantity: i32,

        /// Unit cost
        #[arg(short, long)]
        unit_cost: f64,
    },

    /// Update stock quantity
    UpdateQuantity {
        /// Stock item ID
        #[arg(short, long)]
        id: String,

        /// New quantity
        #[arg(short, long)]
        quantity: i32,
    },
}

fn get_database_url(cli_url: Option<String>) -> String {
    // First try to use the CLI argument if provided
    if let Some(url) = cli_url {
        return url;
    }

    // Then try to use the DATABASE_URL environment variable
    match env::var("DATABASE_URL") {
        Ok(url) => url,
        // Fallback to a default URL
        Err(_) => "postgres://postgres:postgres@localhost/stock_manager".to_string(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Get the database URL from CLI args or environment
    let database_url = get_database_url(cli.database_url);

    match &cli.command {
        Commands::Seed => {
            println!("Seeding database with sample data...");
            let pool = create_pool(&database_url).await?;

            // Create repositories
            let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
            let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));

            // Create service
            let product_service = ProductService::new(product_repo, stock_repo);

            // Add sample products
            let laptop = product_service
                .create_product(
                    "Laptop".to_string(),
                    Some("High-performance laptop".to_string()),
                    "LT-001".to_string(),
                )
                .await?;

            let mouse = product_service
                .create_product(
                    "Mouse".to_string(),
                    Some("Wireless mouse".to_string()),
                    "MS-002".to_string(),
                )
                .await?;

            let keyboard = product_service
                .create_product(
                    "Keyboard".to_string(),
                    Some("Mechanical keyboard".to_string()),
                    "KB-003".to_string(),
                )
                .await?;

            // Add stock items
            product_service
                .add_stock_item(
                    laptop.id,
                    10,
                    "Warehouse A".to_string(),
                    BigDecimal::from_str("999.99")?,
                )
                .await?;

            product_service
                .add_stock_item(
                    mouse.id,
                    20,
                    "Warehouse A".to_string(),
                    BigDecimal::from_str("24.99")?,
                )
                .await?;

            product_service
                .add_stock_item(
                    keyboard.id,
                    15,
                    "Warehouse B".to_string(),
                    BigDecimal::from_str("89.99")?,
                )
                .await?;

            println!("Database seeded successfully.");
        }

        Commands::ListProducts => {
            let pool = create_pool(&database_url).await?;
            let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
            let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
            let product_service = ProductService::new(product_repo, stock_repo);

            let products = product_service.get_all_products().await?;

            println!("Products:");
            println!(
                "{:<36} {:<20} {:<10} {}",
                "ID", "Name", "SKU", "Description"
            );
            println!("{}", "-".repeat(80));

            for product in products {
                println!(
                    "{:<36} {:<20} {:<10} {}",
                    product.id,
                    product.name,
                    product.sku,
                    product.description.unwrap_or_default()
                );
            }
        }

        Commands::AddProduct {
            name,
            sku,
            description,
        } => {
            let pool = create_pool(&database_url).await?;
            let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
            let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
            let product_service = ProductService::new(product_repo, stock_repo);

            let product = product_service
                .create_product(name.clone(), description.clone(), sku.clone())
                .await?;

            println!("Product added successfully:");
            println!("ID: {}", product.id);
            println!("Name: {}", product.name);
            println!("SKU: {}", product.sku);
            println!("Description: {}", product.description.unwrap_or_default());
        }

        Commands::ListStock { product_id } => {
            let pool = create_pool(&database_url).await?;
            let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
            let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
            let product_service = ProductService::new(product_repo, stock_repo);

            let product_id = Uuid::parse_str(product_id)?;
            let product = product_service.get_product(product_id).await?;
            let stock_items = product_service.get_product_stock(product_id).await?;

            println!("Stock for product: {} ({})", product.name, product.sku);
            println!(
                "{:<36} {:<15} {:<10} {:<15} {}",
                "ID", "Location", "Quantity", "Unit Cost", "Last Restocked"
            );
            println!("{}", "-".repeat(90));

            for item in stock_items {
                println!(
                    "{:<36} {:<15} {:<10} {:<15} {}",
                    item.id,
                    item.location,
                    item.quantity,
                    item.unit_cost,
                    item.last_restocked.format("%Y-%m-%d %H:%M:%S")
                );
            }
        }

        Commands::AddStock {
            product_id,
            location,
            quantity,
            unit_cost,
        } => {
            let pool = create_pool(&database_url).await?;
            let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
            let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
            let product_service = ProductService::new(product_repo, stock_repo);

            let product_id = Uuid::parse_str(product_id)?;
            let unit_cost = BigDecimal::from_str(&unit_cost.to_string())?;

            let stock_item = product_service
                .add_stock_item(product_id, *quantity, location.clone(), unit_cost)
                .await?;

            println!("Stock item added successfully:");
            println!("ID: {}", stock_item.id);
            println!("Product ID: {}", stock_item.product_id);
            println!("Location: {}", stock_item.location);
            println!("Quantity: {}", stock_item.quantity);
            println!("Unit Cost: {}", stock_item.unit_cost);
            println!(
                "Last Restocked: {}",
                stock_item.last_restocked.format("%Y-%m-%d %H:%M:%S")
            );
        }

        Commands::UpdateQuantity { id, quantity } => {
            let pool = create_pool(&database_url).await?;
            let product_repo = Arc::new(PostgresProductRepository::new(pool.clone()));
            let stock_repo = Arc::new(PostgresStockRepository::new(pool.clone()));
            let product_service = ProductService::new(product_repo, stock_repo);

            let id = Uuid::parse_str(id)?;

            let stock_item = product_service.update_stock_quantity(id, *quantity).await?;

            println!("Stock quantity updated successfully:");
            println!("ID: {}", stock_item.id);
            println!("New Quantity: {}", stock_item.quantity);
            println!(
                "Last Restocked: {}",
                stock_item.last_restocked.format("%Y-%m-%d %H:%M:%S")
            );
        }
    }

    Ok(())
}

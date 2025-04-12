# Stock Manager

A comprehensive stock management system built with Rust, featuring:

- Domain-driven design architecture
- Clean separation of concerns using the repository pattern
- Both REST API and Web UI (with HTMX) interfaces
- CLI for database setup and management

## Project Structure

The project is organized as a Rust workspace with multiple crates:

- `domain`: Core domain models and repository interfaces
- `infrastructure`: Database implementations of the repositories
- `application`: Business logic and service layer
- `api-server`: REST API server
- `web-server`: Web UI with HTMX integration
- `cli`: Command-line interface for database management

## Getting Started

You can run this application either with Docker (recommended) or directly with Rust.

### Using Docker (Recommended)

#### Prerequisites

- Docker and Docker Compose

#### Setup and Run

1. Clone the repository
2. Make the startup script executable:

```bash
chmod +x start.sh stop.sh
```

3. Start the application with a single command:

```bash
./start.sh
```

This will:
- Start a PostgreSQL database
- Run migrations and seed the database
- Start both the API server and Web UI server

To stop the application:

```bash
./stop.sh
```

### Using Rust Directly

#### Prerequisites

- Rust (2024 edition)
- PostgreSQL database

#### Setup

1. Clone the repository
2. Create a PostgreSQL database named `stock_manager`
3. Setup environment variables (or use defaults):

```bash
export DATABASE_URL=postgres://username:password@localhost/stock_manager
```

4. Run the database setup:

```bash
cargo run --bin stock-cli -- setup
```

5. Seed the database with sample data (optional):

```bash
cargo run --bin stock-cli -- seed
```

#### Running the Application

Start the REST API server:

```bash
cargo run --bin stock-api-server
```

Start the Web UI server:

```bash
cargo run --bin stock-web-server
```

## API Endpoints

The REST API is available at `http://localhost:8080/api`:

### Products

- `GET /api/products` - List all products
- `GET /api/products/{id}` - Get a product by ID
- `POST /api/products` - Create a new product
- `PUT /api/products/{id}` - Update a product
- `DELETE /api/products/{id}` - Delete a product

### Stock

- `GET /api/products/{id}/stock` - Get stock items for a product
- `POST /api/stock` - Create a new stock item
- `PUT /api/stock/{id}/quantity` - Update a stock item's quantity

## Web Interface

The web interface is available at `http://localhost:8081`:

- `/` - Home page
- `/products` - Product management
- `/stock` - Stock management

## CLI Commands

The CLI tool provides several commands for database management:

```bash
# Setup the database
cargo run --bin stock-cli -- setup

# Seed the database with sample data
cargo run --bin stock-cli -- seed

# List all products
cargo run --bin stock-cli -- list-products

# Add a new product
cargo run --bin stock-cli -- add-product --name "Product Name" --sku "SKU-123" --description "Description"

# List stock for a product
cargo run --bin stock-cli -- list-stock --product-id "uuid"

# Add a stock item
cargo run --bin stock-cli -- add-stock --product-id "uuid" --location "Warehouse A" --quantity 10 --unit-cost 99.99

# Update stock quantity
cargo run --bin stock-cli -- update-quantity --id "uuid" --quantity 20
```

## Database Schema

The database consists of two main tables:

### Products Table

| Column      | Type      | Description                   |
|-------------|-----------|-------------------------------|
| id          | UUID      | Primary key                   |
| name        | VARCHAR   | Product name                  |
| description | TEXT      | Optional product description  |
| sku         | VARCHAR   | Stock keeping unit (unique)   |
| created_at  | TIMESTAMP | Creation timestamp            |
| updated_at  | TIMESTAMP | Last update timestamp         |

### Stock Items Table

| Column         | Type      | Description                   |
|----------------|-----------|-------------------------------|
| id             | UUID      | Primary key                   |
| product_id     | UUID      | Foreign key to products table |
| quantity       | INTEGER   | Current quantity              |
| location       | VARCHAR   | Storage location              |
| unit_cost      | DECIMAL   | Cost per unit                 |
| last_restocked | TIMESTAMP | Last restock timestamp        |

## License

MIT

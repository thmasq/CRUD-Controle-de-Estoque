# Stock Manager

Stock Manager is a warehouse inventory management system built with Rust. It allows businesses to track products across multiple warehouses, manage inventory levels, and record stock transactions for accurate inventory control.

## Features

- **Product Management**: Create, edit, and categorize products with SKUs
- **Warehouse Management**: Manage multiple warehouse locations
- **Inventory Tracking**: Track stock levels across warehouses
- **Transaction History**: Record and view stock movements (in, out, adjustments)
- **Dashboard**: View key metrics and low stock alerts
- **User Authentication**: Role-based access control (Manager/Seller roles)

## Technologies

- **Backend**: Rust with Actix-Web framework
- **Database**: PostgreSQL with Diesel ORM
- **Frontend**: HTMX for interactivity, Tailwind CSS for styling
- **Authentication**: JWT tokens

## Prerequisites

- Rust (latest stable version)
- PostgreSQL (v13+)
- Docker and Docker Compose (optional, for containerized deployment)

## Getting Started

### Option 1: Using Docker Compose (Recommended)

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/stock-manager.git
   cd stock-manager
   ```

2. Start the application:
   ```
   docker-compose up
   ```
   
3. Access the application at http://localhost:8080

### Option 2: Manual Setup

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/stock-manager.git
   cd stock-manager
   ```

2. Set up the database:
   ```
   createdb stockmanager
   psql -d stockmanager -f init.sql
   ```

3. Create a `.env` file in the project root:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost/stockmanager
   RUST_LOG=stock_web_server=debug,actix_web=info
   HOST=127.0.0.1
   PORT=8080
   JWT_SECRET=your_secret_key_here
   ENABLE_REGISTRATION=true  # Set to false in production
   ```

4. Build and run the application:
   ```
   cargo build --release
   ./target/release/stock-web-server
   ```

5. Access the application at http://localhost:8080

## Default Login

When you first start the application with registration enabled, create an account at `/auth/register`.

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `HOST`: Bind address (default: 127.0.0.1)
- `PORT`: HTTP port (default: 8080)
- `JWT_SECRET`: Secret key for JWT token generation
- `ENABLE_REGISTRATION`: Set to "true" to allow user registration (disabled by default)
- `RUST_LOG`: Logging configuration

## Project Structure

- `crates/domain`: Core domain entities and repository traits
- `crates/application`: Business logic and services
- `crates/infrastructure`: Database implementations of repositories
- `crates/web-server`: Web interface with Actix Web
- `crates/api-server`: RESTful API (WIP)
- `crates/cli`: Command line interface (WIP)

## License

AGPLv3

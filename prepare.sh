#!/bin/bash
set -e

echo "🔍 Preparing SQLx offline mode..."

# Check if sqlx-cli is installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing sqlx-cli..."
    cargo install sqlx-cli
fi

# Start a temporary Postgres container
echo "Starting temporary PostgreSQL for SQLx prepare..."
CONTAINER_ID=$(docker run -d \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_DB=stock_manager \
    -p 5433:5432 \
    postgres:16-alpine)

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
sleep 5
until docker exec $CONTAINER_ID pg_isready -U postgres -d stock_manager > /dev/null 2>&1; do
    echo "Waiting for PostgreSQL..."
    sleep 1
done

# Set DATABASE_URL to point to our temporary container
export DATABASE_URL="postgres://postgres:postgres@localhost:5433/stock_manager"

echo "Running migrations..."
sqlx migrate run

echo "Preparing SQLx data..."
cargo sqlx prepare --workspace

echo "Cleaning up..."
docker stop $CONTAINER_ID
docker rm $CONTAINER_ID

echo "✅ SQLx offline mode prepared successfully!"
echo "A sqlx-data.json file has been created in the project root."
echo "You can now build the Docker image without a database connection."

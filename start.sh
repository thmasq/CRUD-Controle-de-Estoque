#!/bin/bash
set -e

echo "🚀 Starting Stock Manager..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
  echo "⚠️  Docker is not running. Please start Docker and try again."
  exit 1
fi

# Build and start the database container first
echo "🏗️  Starting database container..."
docker-compose up -d db

# Wait for the database to be ready
echo "⏳ Waiting for database to be ready..."
while ! docker-compose exec db pg_isready -U "${POSTGRES_USER:-postgres}" -d "${POSTGRES_DB:-stock_manager}" >/dev/null 2>&1; do
  echo "Waiting for database connection..."
  sleep 2
done

# Start seed container to add sample data
echo "🌱 Seeding the database..."
docker-compose up -d seed

# Wait for seed container to complete
echo "⏳ Waiting for seed process to complete..."
docker-compose run --rm seed

# Start the API and Web containers
echo "🏗️  Starting API and Web services..."
docker-compose up -d api web

echo "⏳ Waiting for services to be ready..."
sleep 5

echo "✅ Stock Manager is now running!"
echo "📊 API is available at http://localhost:${API_PORT:-8080}/api"
echo "🌐 Web UI is available at http://localhost:${WEB_PORT:-8081}"
echo ""
echo "You can use the following commands:"
echo "- 'docker-compose logs -f' to view logs"
echo "- 'docker-compose down' to stop all services"
echo "- './check.sh' to check service status"

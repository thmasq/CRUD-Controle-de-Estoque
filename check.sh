#!/bin/bash
set -e

echo "🔍 Checking Stock Manager status..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
  echo "⚠️  Docker is not running. Please start Docker and try again."
  exit 1
fi

# Check if containers are running
API_RUNNING=$(docker ps -q -f name=stock-manager-api)
WEB_RUNNING=$(docker ps -q -f name=stock-manager-web)
DB_RUNNING=$(docker ps -q -f name=stock-manager-db)

echo "🐳 Container status:"
if [ -z "$DB_RUNNING" ]; then
  echo "❌ Database container is not running"
else
  echo "✅ Database container is running"
fi

if [ -z "$API_RUNNING" ]; then
  echo "❌ API container is not running"
else
  echo "✅ API container is running"
  
  # Check if API is responding
  if curl -s http://localhost:${API_PORT:-8080}/api/products > /dev/null; then
    echo "  ✅ API is responding to requests"
  else
    echo "  ❌ API is not responding to requests"
  fi
fi

if [ -z "$WEB_RUNNING" ]; then
  echo "❌ Web UI container is not running"
else
  echo "✅ Web UI container is running"
  
  # Check if Web UI is responding
  if curl -s http://localhost:${WEB_PORT:-8081} > /dev/null; then
    echo "  ✅ Web UI is responding to requests"
  else
    echo "  ❌ Web UI is not responding to requests"
  fi
fi

echo ""
echo "🌐 Network information:"
echo "  Network name: stock-network"
docker network inspect stock-manager_stock-network --format='  Container IPs: {{range $i, $c := .Containers}}{{if $i}}, {{end}}{{$c.Name}}: {{$c.IPv4Address}}{{end}}' 2>/dev/null || echo "  Network not found or no containers connected"

echo ""
echo "📊 API URL: http://localhost:${API_PORT:-8080}/api"
echo "🌐 Web UI URL: http://localhost:${WEB_PORT:-8081}"

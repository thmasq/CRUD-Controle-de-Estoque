#!/bin/bash
set -e

echo "🌱 Seeding database with sample data..."

until PGPASSWORD=$POSTGRES_PASSWORD psql -h db -U $POSTGRES_USER -d $POSTGRES_DB -c '\q'; do
  echo "Waiting for database connection..."
  sleep 2
done

TABLE_COUNT=$(PGPASSWORD=$POSTGRES_PASSWORD psql -h db -U $POSTGRES_USER -d $POSTGRES_DB -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';")
if [ "$TABLE_COUNT" -lt 2 ]; then
  echo "Tables not yet created. Executing init.sql..."
  PGPASSWORD=$POSTGRES_PASSWORD psql -h db -U $POSTGRES_USER -d $POSTGRES_DB -f /app/init.sql
fi

echo "Executing seed.sql..."
PGPASSWORD=$POSTGRES_PASSWORD psql -h db -U $POSTGRES_USER -d $POSTGRES_DB -f /app/seed.sql

echo "✅ Database seeded successfully!"

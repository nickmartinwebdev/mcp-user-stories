#!/bin/bash

# Simple database setup script for CI and local development
set -e

# Configuration
DB_PATH="${1:-test.db}"
DATABASE_URL="sqlite://${DB_PATH}"
PREPARE_QUERIES="${2:-}"

echo "🔧 Setting up database: $DB_PATH"

# Install SQLx CLI if needed
if ! command -v sqlx &> /dev/null; then
    echo "Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features sqlite
fi

# Clean up existing database
[ -f "$DB_PATH" ] && rm "$DB_PATH"

# Create database and run migrations
echo "Creating database and running migrations..."
sqlite3 "$DB_PATH" "SELECT 1" > /dev/null
sqlx migrate run --database-url "$DATABASE_URL"

# Verify schema
TABLES=$(sqlite3 "$DB_PATH" ".tables")
if ! echo "$TABLES" | grep -q "user_stories\|acceptance_criteria"; then
    echo "❌ Schema verification failed"
    exit 1
fi

# Prepare queries if requested
if [ "$PREPARE_QUERIES" = "--prepare" ]; then
    echo "Preparing SQLx queries..."
    cargo sqlx prepare --database-url "$DATABASE_URL"
fi

echo "✅ Database setup complete: $DB_PATH"

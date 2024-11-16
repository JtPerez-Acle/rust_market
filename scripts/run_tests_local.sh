#!/bin/bash

# Exit on any error
set -e

echo "🚀 Starting local test environment..."

# Function to wait for PostgreSQL to be ready
wait_for_postgres() {
    echo "⏳ Waiting for PostgreSQL to be ready..."
    for i in {1..30}; do
        if docker compose -f docker-compose.test.yml exec -T db_test pg_isready -U postgres > /dev/null 2>&1; then
            echo "✅ PostgreSQL is ready!"
            return 0
        fi
        echo -n "."
        sleep 1
    done
    echo "❌ PostgreSQL did not become ready in time"
    exit 1
}

# Function to run database migrations
run_migrations() {
    echo "🔄 Running database migrations..."
    export DATABASE_URL="postgres://postgres:postgres@localhost:5433/rust_market_test"
    
    if ! command -v diesel &> /dev/null; then
        echo "📦 Installing diesel_cli..."
        cargo install diesel_cli --no-default-features --features postgres
    fi
    
    diesel migration run || {
        echo "❌ Migration failed"
        exit 1
    }
    echo "✅ Migrations completed successfully!"
}

# Cleanup function
cleanup() {
    echo "🧹 Cleaning up..."
    docker compose -f docker-compose.test.yml down
    if [ "$?" -ne 0 ]; then
        echo "⚠️  Warning: Cleanup failed, you may need to manually remove containers"
    fi
}

# Set up cleanup trap
trap cleanup EXIT

# 1. Start the test database
echo "🗄️  Starting PostgreSQL container..."
docker compose -f docker-compose.test.yml up -d db_test

# 2. Wait for PostgreSQL to be ready
wait_for_postgres

# 3. Run migrations
run_migrations

# 4. Run the tests with detailed output
echo "🧪 Running tests with detailed output..."
RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1 $@

# Note: Cleanup will be handled by the trap

echo "✨ Test run completed!"

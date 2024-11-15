#!/bin/bash

# Database configuration
DB_NAME="rust_market_test"
DB_USER="jtdev"
DB_PASS="dev1998"

# Drop existing database if it exists
dropdb --if-exists -U "$DB_USER" "$DB_NAME"

# Create new database
createdb -U "$DB_USER" "$DB_NAME"

# Run migrations
diesel migration run --database-url="postgres://${DB_USER}:${DB_PASS}@localhost/${DB_NAME}"

# Create test logs directory
mkdir -p tests/logs

echo "Test database setup complete!"

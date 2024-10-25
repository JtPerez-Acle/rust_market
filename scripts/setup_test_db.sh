#!/bin/bash

# Exit on any error
set -e

echo "Setting up test database..."

# Store the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Prompt for database credentials if not provided
read -p "Enter database username (default: jtdev): " DB_USER
DB_USER=${DB_USER:-jtdev}

read -s -p "Enter database password: " DB_PASS
echo
DB_PASS=${DB_PASS:-dev1998}

DB_NAME="rust_market_test"

# Export for psql commands
export PGPASSWORD="$DB_PASS"

# Function to terminate active connections to the database
terminate_db_connections() {
    echo "Terminating active connections to the database..."
    psql -U "$DB_USER" -h localhost -d postgres -c "
        SELECT pg_terminate_backend(pid)
        FROM pg_stat_activity
        WHERE datname = '$DB_NAME' AND pid <> pg_backend_pid();
    " > /dev/null 2>&1
}

echo "Attempting to drop database if it exists..."

# Terminate active connections before dropping the database
terminate_db_connections

# Drop the database
dropdb -U "$DB_USER" -h localhost "$DB_NAME" --if-exists

echo "Creating new test database..."
createdb -U "$DB_USER" -h localhost "$DB_NAME"

# Create .env.test file
cat > "${PROJECT_ROOT}/.env.test" << EOL
DATABASE_URL_TEST=postgres://${DB_USER}:${DB_PASS}@localhost/${DB_NAME}
RUST_LOG=debug
EOL

echo "Running migrations..."
cd "${PROJECT_ROOT}" && diesel migration run --database-url "postgres://${DB_USER}:${DB_PASS}@localhost/${DB_NAME}"

echo "Testing database connection..."
if psql -U "$DB_USER" -h localhost "$DB_NAME" -c "SELECT 1" > /dev/null 2>&1; then
    echo "✅ Database setup complete and connection successful!"
else
    echo "❌ Warning: Could not connect to database. Please check your credentials."
    exit 1
fi

# Unset the password environment variable
unset PGPASSWORD

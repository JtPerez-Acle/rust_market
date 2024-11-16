#!/bin/bash

# Exit on any error
set -e

echo "Setting up test database..."

# Store the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Default values
DB_USER="jtdev"
DB_PASS="dev1998"
DB_NAME="rust_market_test"
DB_HOST="localhost"
USE_DOCKER=false

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --docker) USE_DOCKER=true; DB_HOST="localhost"; shift ;;
        --user) DB_USER="$2"; shift 2 ;;
        --password) DB_PASS="$2"; shift 2 ;;
        --host) DB_HOST="$2"; shift 2 ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
done

if [ "$USE_DOCKER" = true ]; then
    echo "Using Docker setup..."
    
    # Check if Docker is running
    if ! docker info > /dev/null 2>&1; then
        echo " Error: Docker is not running or not installed"
        exit 1
    fi

    # Start PostgreSQL container if not already running
    if ! docker compose -f "${PROJECT_ROOT}/docker-compose.test.yml" ps | grep -q "db_test"; then
        echo "Starting PostgreSQL container..."
        docker compose -f "${PROJECT_ROOT}/docker-compose.test.yml" up -d
        
        # Wait for PostgreSQL to be ready
        echo "Waiting for PostgreSQL to be ready..."
        for i in {1..30}; do
            if docker exec $(docker compose -f "${PROJECT_ROOT}/docker-compose.test.yml" ps -q db_test) pg_isready -U ${DB_USER} > /dev/null 2>&1; then
                break
            fi
            echo -n "."
            sleep 1
        done
        echo ""
    fi
else
    echo "Using local PostgreSQL setup..."
    # Export for psql commands
    export PGPASSWORD="$DB_PASS"

    # Function to terminate active connections to the database
    terminate_db_connections() {
        echo "Terminating active connections to the database..."
        psql -U "$DB_USER" -h "$DB_HOST" -d postgres -c "
            SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = '$DB_NAME' AND pid <> pg_backend_pid();
        " > /dev/null 2>&1 || true
    }

    echo "Attempting to drop database if it exists..."
    # Terminate active connections before dropping the database
    terminate_db_connections
    dropdb -U "$DB_USER" -h "$DB_HOST" "$DB_NAME" --if-exists || true
    
    echo "Creating new test database..."
    createdb -U "$DB_USER" -h "$DB_HOST" "$DB_NAME"
fi

# Create .env.test file
cat > "${PROJECT_ROOT}/.env.test" << EOL
DATABASE_URL_TEST=postgres://${DB_USER}:${DB_PASS}@${DB_HOST}/${DB_NAME}
RUST_LOG=debug
EOL

echo "Running migrations..."
cd "${PROJECT_ROOT}"

# Retry logic for running migrations
MAX_RETRIES=5
RETRY_COUNT=0
while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if diesel migration run --database-url "postgres://${DB_USER}:${DB_PASS}@${DB_HOST}/${DB_NAME}"; then
        break
    fi
    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo "Migration failed, retrying in 2 seconds... (Attempt $RETRY_COUNT of $MAX_RETRIES)"
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo " Failed to run migrations after $MAX_RETRIES attempts"
    exit 1
fi

echo "Testing database connection..."
if [ "$USE_DOCKER" = true ]; then
    docker exec $(docker compose -f "${PROJECT_ROOT}/docker-compose.test.yml" ps -q db_test) psql -U "$DB_USER" -d "$DB_NAME" -c "SELECT 1" > /dev/null 2>&1
else
    psql -U "$DB_USER" -h "$DB_HOST" "$DB_NAME" -c "SELECT 1" > /dev/null 2>&1
fi

if [ $? -eq 0 ]; then
    echo " Database setup complete and connection successful!"
else
    echo " Warning: Could not connect to database. Please check your credentials."
    exit 1
fi

# Unset the password environment variable if using local PostgreSQL
if [ "$USE_DOCKER" = false ]; then
    unset PGPASSWORD
fi

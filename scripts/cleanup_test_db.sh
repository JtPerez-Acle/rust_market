#!/bin/bash

# Exit on any error
set -e

echo "Cleaning up test database..."

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

    # Get container ID
    CONTAINER_ID=$(docker compose -f "${PROJECT_ROOT}/docker-compose.test.yml" ps -q db_test)
    
    if [ -z "$CONTAINER_ID" ]; then
        echo "Test database container is not running"
        exit 0
    fi

    echo "Cleaning up test database in Docker container..."
    docker exec $CONTAINER_ID psql -U ${DB_USER} -d ${DB_NAME} -c "
        DO \$\$ 
        DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END \$\$;
    "
else
    echo "Using local PostgreSQL setup..."
    # Export for psql commands
    export PGPASSWORD="$DB_PASS"

    echo "Cleaning up test database..."
    psql -U "$DB_USER" -h "$DB_HOST" -d "$DB_NAME" -c "
        DO \$\$ 
        DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
                EXECUTE 'TRUNCATE TABLE ' || quote_ident(r.tablename) || ' CASCADE';
            END LOOP;
        END \$\$;
    "

    # Unset password environment variable
    unset PGPASSWORD
fi

echo "Test database cleanup complete!"

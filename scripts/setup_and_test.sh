#!/bin/bash

# Exit on any error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_NAME="rust_market_test"
DB_PORT=5432
SERVICE_PORT=8080
MAX_RETRIES=3
TIMEOUT=10

# Logging function
log() {
    echo -e "${YELLOW}$(date '+%Y-%m-%d %H:%M:%S') - $1${NC}"
}

# Error logging function
error() {
    echo -e "${RED}$(date '+%Y-%m-%d %H:%M:%S') - ERROR: $1${NC}"
}

# Success logging function
success() {
    echo -e "${GREEN}$(date '+%Y-%m-%d %H:%M:%S') - SUCCESS: $1${NC}"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check if postgres is running with timeout
check_postgres() {
    local timeout=$1
    local start_time=$(date +%s)
    while true; do
        if pg_isready -h localhost -p $DB_PORT > /dev/null 2>&1; then
            return 0
        fi
        
        local current_time=$(date +%s)
        if [ $((current_time - start_time)) -ge $timeout ]; then
            return 1
        fi
        sleep 1
    done
}

# Function to verify database connection
verify_db_connection() {
    local retries=0
    while [ $retries -lt $MAX_RETRIES ]; do
        if PGPASSWORD=$DB_PASSWORD psql -h localhost -U $DB_USER -d $DB_NAME -c '\q' 2>/dev/null; then
            return 0
        fi
        retries=$((retries + 1))
        log "Retry $retries/$MAX_RETRIES: Waiting for database connection..."
        sleep 2
    done
    return 1
}

# Function to handle cleanup on script exit
cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ]; then
        error "Script failed with exit code $exit_code"
        error "Check the logs above for more details"
    fi
    exit $exit_code
}

# Register cleanup function
trap cleanup EXIT

# Main setup process
main() {
    log "Starting setup process..."

    # Step 1: Check system dependencies
    log "Checking system dependencies..."
    
    if ! command_exists psql; then
        log "Installing PostgreSQL..."
        if ! sudo apt update && sudo apt install -y postgresql postgresql-contrib; then
            error "Failed to install PostgreSQL"
            return 1
        fi
    fi

    if ! command_exists cargo; then
        log "Installing Rust..."
        if ! curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; then
            error "Failed to install Rust"
            return 1
        fi
        source $HOME/.cargo/env
    fi

    # Step 2: Ensure PostgreSQL is running
    log "Checking PostgreSQL service..."
    if ! check_postgres $TIMEOUT; then
        log "Starting PostgreSQL service..."
        if ! sudo service postgresql start; then
            error "Failed to start PostgreSQL"
            return 1
        fi
        
        log "Waiting for PostgreSQL to be ready..."
        if ! check_postgres $TIMEOUT; then
            error "PostgreSQL failed to start within ${TIMEOUT} seconds"
            return 1
        fi
    fi
    success "PostgreSQL is running"

    # Step 3: Set up database
    log "Setting up database..."
    
    # Set up postgres user
    if ! sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';" 2>/dev/null; then
        error "Failed to set postgres user password"
        return 1
    fi

    # Drop and recreate database
    log "Recreating database..."
    if ! sudo -u postgres psql -c "DROP DATABASE IF EXISTS $DB_NAME;" 2>/dev/null; then
        error "Failed to drop database"
        return 1
    fi
    
    if ! sudo -u postgres psql -c "CREATE DATABASE $DB_NAME;" 2>/dev/null; then
        error "Failed to create database"
        return 1
    fi

    # Verify database connection
    log "Verifying database connection..."
    if ! verify_db_connection; then
        error "Could not establish database connection"
        return 1
    fi
    success "Database setup completed"

    # Step 4: Install Rust dependencies
    log "Installing Rust dependencies..."
    if ! command_exists diesel && ! cargo install diesel_cli --no-default-features --features postgres; then
        error "Failed to install Diesel CLI"
        return 1
    fi

    # Step 5: Set up environment
    log "Setting up environment..."
    export DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
    export DATABASE_URL_TEST="postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
    export RUST_LOG="debug"
    export RUST_BACKTRACE=1

    # Create/update .env file
    cat > .env << EOF
DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME
DATABASE_URL_TEST=postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME
RUST_LOG=debug
RUST_BACKTRACE=1
EOF

    # Step 6: Clean up old migrations
    log "Cleaning up old migrations..."
    rm -rf migrations/2024-10-25-171428_create_market_tables

    # Step 7: Run migrations
    log "Running database migrations..."
    if ! diesel migration run; then
        error "Failed to run migrations"
        return 1
    fi
    success "Migrations completed"

    # Step 8: Build project
    log "Building project..."
    if ! cargo build; then
        error "Failed to build project"
        return 1
    fi
    success "Build completed"

    # Step 9: Run tests
    log "Running tests..."
    if ! cargo test -- --test-threads=1; then
        error "Tests failed"
        return 1
    fi
    success "Tests completed"

    success "Setup completed successfully!"
    echo
    echo -e "${GREEN}Next steps:${NC}"
    echo -e "1. Run the service: ${YELLOW}cargo run${NC}"
    echo -e "2. Run tests: ${YELLOW}cargo test${NC}"
    echo -e "3. Access the API at: ${YELLOW}http://localhost:8080${NC}"
    
    return 0
}

# Execute main function
main

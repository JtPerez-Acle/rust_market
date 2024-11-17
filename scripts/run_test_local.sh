#!/bin/bash

set -e

# Configuration
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_NAME="rust_market_test"
DB_PORT=5432
SERVICE_PORT=8080

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting test environment setup...${NC}"

# Function to check if postgres is running
check_postgres() {
    pg_isready -h localhost -p $DB_PORT > /dev/null 2>&1
    return $?
}

# Function to check if our service is running
check_service() {
    curl -s http://localhost:$SERVICE_PORT/health > /dev/null
    return $?
}

# Function to check if Docker is running
check_docker() {
    docker info >/dev/null 2>&1
    return $?
}

# Check if Docker is running
if ! check_docker; then
    echo -e "${RED}Docker is not running. Please start Docker and try again.${NC}"
    exit 1
fi

# Check if PostgreSQL is running via Docker
if ! check_postgres; then
    echo -e "${YELLOW}PostgreSQL is not running. Starting Docker environment...${NC}"
    docker-compose up -d db
    
    # Wait for PostgreSQL to start
    echo -e "${YELLOW}Waiting for PostgreSQL to start...${NC}"
    for i in {1..30}; do
        if check_postgres; then
            break
        fi
        sleep 1
        if [ $i -eq 30 ]; then
            echo -e "${RED}Failed to start PostgreSQL. Check Docker logs with: docker-compose logs db${NC}"
            exit 1
        fi
    done
fi

# Set environment variables
export DATABASE_URL_TEST="postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME"
export RUST_LOG="debug"
export RUST_BACKTRACE=1

# Run database migrations
echo -e "${YELLOW}Running database migrations...${NC}"
if ! command -v diesel &> /dev/null; then
    echo -e "${YELLOW}Installing diesel CLI...${NC}"
    cargo install diesel_cli --no-default-features --features postgres
fi
diesel migration run

# Build the project
echo -e "${YELLOW}Building project...${NC}"
cargo build

# Start the service in the background
echo -e "${YELLOW}Starting service...${NC}"
cargo run &
SERVICE_PID=$!

# Wait for the service to start
echo -e "${YELLOW}Waiting for service to start...${NC}"
for i in {1..30}; do
    if check_service; then
        break
    fi
    sleep 1
    if [ $i -eq 30 ]; then
        echo -e "${RED}Service failed to start. Check the logs for more information.${NC}"
        kill $SERVICE_PID 2>/dev/null || true
        exit 1
    fi
done

# Run the tests
echo -e "${YELLOW}Running tests...${NC}"
cargo test -- --test-threads=1

# Cleanup
echo -e "${YELLOW}Cleaning up...${NC}"
kill $SERVICE_PID 2>/dev/null || true

echo -e "${GREEN}Tests completed!${NC}"

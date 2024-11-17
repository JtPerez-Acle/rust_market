#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting CI test environment setup...${NC}"

# Check if required environment variables are set
required_vars=("DATABASE_URL_TEST" "PORT")
for var in "${required_vars[@]}"; do
    if [ -z "${!var}" ]; then
        echo -e "${RED}Error: $var is not set${NC}"
        exit 1
    fi
done

# Run database migrations
echo -e "${YELLOW}Running database migrations...${NC}"
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
    if curl -s "http://localhost:$PORT/health" > /dev/null; then
        break
    fi
    sleep 1
done

# Run the tests
echo -e "${YELLOW}Running tests...${NC}"
RUST_LOG=debug RUST_BACKTRACE=1 cargo test -- --test-threads=1

# Cleanup
echo -e "${YELLOW}Cleaning up...${NC}"
kill $SERVICE_PID

echo -e "${GREEN}Tests completed!${NC}"

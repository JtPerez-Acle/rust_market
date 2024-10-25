# Testing Guide for `rust_market` Project

This guide explains how to set up and run tests for the `rust_market` project. It covers creating a test database, configuring the environment, running tests, and understanding different edge cases.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Setting Up the Test Database](#setting-up-the-test-database)
3. [Configuring the Environment](#configuring-the-environment)
4. [Running the Tests](#running-the-tests)
5. [Understanding the Tests](#understanding-the-tests)
6. [Handling Edge Cases](#handling-edge-cases)
7. [Additional Tips](#additional-tips)

## Prerequisites

- **Rust** installed on your system.
- **Cargo** package manager.
- **PostgreSQL** installed and running locally.
- Basic understanding of Rust and Diesel ORM.

## Setting Up the Test Database

1. **Create a Test Database:**

   Open your terminal and run the following command to create a new PostgreSQL database for testing:

   ```bash
   createdb rust_market_test
   ```

2. **Set Up Database Schema:**

   Run Diesel migrations to set up the database schema in the test database:

   ```bash
   diesel migration run --database-url postgres://username:password@localhost/rust_market_test
   ```

   Replace `username` and `password` with your PostgreSQL credentials.

## Configuring the Environment

1. **Create a `.env.test` File:**

   In the project's root directory, create a file named `.env.test` with the following content:

   ```env
   DATABASE_URL_TEST=postgres://username:password@localhost/rust_market_test
   ```

   Ensure that this file contains the connection string to your test database.

2. **Modify Test Setup to Use `.env.test`:**

   In your test scripts, load `.env.test`:

   ```rust
   dotenv::from_filename(".env.test").ok();
   ```

   And retrieve `DATABASE_URL_TEST`:

   ```rust
   let test_database_url = env::var("DATABASE_URL_TEST")
       .expect("DATABASE_URL_TEST must be set in .env.test");
   ```

3. **Passing the Test Database URL:**

   When establishing the connection pool in tests:

   ```rust
   let pool = db::establish_connection_pool(Some(&test_database_url))
       .expect("Failed to create pool");
   ```

## Running the Tests

To run all tests, execute the following command from the project's root directory:

```bash
cargo test
```

This will compile the tests and run them, providing output on passed and failed tests.

To see detailed output and prevent Rust from capturing standard output, use:

```bash
cargo test -- --nocapture
```

## Understanding the Tests

### Test Modules

The tests are organized into modules within the `tests` directory:

- `models_tests.rs`: Tests for database models and CRUD operations
- `performance_tests.rs`: Tests for system performance and benchmarking
- `db_tests.rs`: Tests related to database connections
- `handlers_tests.rs`: Tests for HTTP request handlers
- `errors_tests.rs`: Tests for custom error handling

### Running Specific Test Modules

To run specific test modules:

```bash
# Run model tests
cargo test --test models_tests -- --nocapture

# Run performance tests
cargo test --test performance_tests -- --nocapture
```

### Models Tests (`models_tests.rs`)

Tests CRUD operations and model relationships:

```bash
cargo test --test models_tests -- --show-output
```

Key test cases:
- `test_create_new_user`: Tests user creation and validation
- `test_insert_duplicate_user`: Tests unique constraint handling
- `test_create_order_with_items`: Tests complex relationships and transactions
- `test_update_product_stock`: Tests inventory management

Example output:
```
running 4 tests
test test_create_new_user ... ok
test test_insert_duplicate_user ... ok
test test_create_order_with_items ... ok
test test_update_product_stock ... ok
```

### Performance Tests (`performance_tests.rs`)

Measures system performance metrics:

```bash
cargo test --test performance_tests -- --nocapture
```

Key metrics displayed:
- Average write time per operation
- Average read time per operation
- Total write/read times
- Operations per second

Example output:
```
Performance Test Results:
-------------------------
Total iterations: 1000
Average write time: 1.23ms
Average read time: 0.89ms
Total write time: 1.23s
Total read time: 0.89s
Write operations/sec: 813
Read operations/sec: 1123
```

### Viewing Detailed Test Results

For comprehensive test output with timing information:

```bash
# Show test output with timing
RUST_TEST_THREADS=1 cargo test -- --nocapture --test-threads=1

# Show only failed tests
cargo test -- --failed

# Show test output with debug information
RUST_LOG=debug cargo test -- --nocapture
```

### Performance Monitoring

To monitor database performance during tests:

1. Enable logging in `.env.test`:
```env
RUST_LOG=debug
```

2. Run tests with performance logging:
```bash
RUST_LOG=debug cargo test --test performance_tests -- --nocapture
```

Example performance log output:
```
[DEBUG] Write operation batch 1-100: Avg time 1.2ms
[DEBUG] Read operation batch 1-100: Avg time 0.8ms
[DEBUG] Database connection pool usage: 45%
[DEBUG] Query execution times:
- Insert operations: 1.1ms avg
- Select operations: 0.7ms avg
- Update operations: 1.3ms avg
```

### Test Database Metrics

Important metrics to monitor:
- Connection pool utilization
- Query execution times
- Transaction throughput
- Error rates
- Response time distribution

To view these metrics while running tests:

```bash
# Enable detailed metrics
RUST_LOG=debug,diesel=debug cargo test --test performance_tests -- --nocapture
```

### Analyzing Test Results

The test results provide insights into:
- System performance under load
- Database operation efficiency
- Error handling effectiveness
- Data integrity maintenance

Key performance indicators:
1. Response times:
   - < 10ms: Excellent
   - 10-50ms: Good
   - > 50ms: Needs investigation

2. Error rates:
   - < 0.1%: Acceptable
   - > 0.1%: Needs investigation

3. Transaction throughput:
   - Write operations: > 500/sec
   - Read operations: > 1000/sec

## Handling Edge Cases

### Missing Environment Variables

- **Test:** Verify that the application handles missing or invalid `DATABASE_URL` gracefully.
- **Solution:** The test `test_establish_connection_pool_failure` removes the `DATABASE_URL` variable and checks that establishing the connection pool fails as expected.

### Database Connection Failure

- **Test:** Simulate a database connection error (e.g., database service is down).
- **Solution:** Temporarily shut down the PostgreSQL service and run the tests to ensure proper error handling is in place.

### Invalid Data Insertion

- **Test:** Attempt to insert invalid or duplicate data into the database.
- **Solution:** Write tests that try to insert data violating constraints and assert that the appropriate errors are returned.

### Unauthorized Access

- **Test:** Access protected endpoints without authentication.
- **Solution:** In `handlers_tests.rs`, write tests to ensure that unauthorized requests receive the correct HTTP status codes.

## Additional Tips

- **Isolation:** Each test should be independent. Use transactions to prevent side effects.
- **Cleanup:** If not using transactions, ensure that test data is cleaned up after tests run.
- **Logging:** Enable logging in your tests to help with debugging.
- **Environment Variables:** Be cautious with environment variables in tests to avoid affecting development or production data.

## Conclusion

By following this guide, you can effectively run and understand tests in the `rust_market` project. Testing is crucial for maintaining code quality and ensuring that your application behaves as expected under various scenarios.

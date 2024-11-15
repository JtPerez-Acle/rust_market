# Testing Guide for Rust Market

This guide explains how to run and understand the test suite for the Rust Market project.

## Prerequisites

Before running the tests, ensure you have:

1. Rust and Cargo installed
2. PostgreSQL running locally
3. `.env.test` file configured with:
   ```env
   DATABASE_URL_TEST=postgres://username:password@localhost/rust_market_test
   RUST_LOG=debug
   ```

## Setting Up the Test Environment

1. Run the setup script:
   ```bash
   ./scripts/setup_test_db.sh
   ```

   This will:
   - Create the test database
   - Run migrations
   - Create the test logs directory

### Run All Tests

```bash
cargo test
```


### Run Specific Test Categories

1. Database Tests:
   ```bash
   cargo test --test db_tests
   ```
   Expected output: Tests for database connection pool and error handling

2. Model Tests:
   ```bash
   cargo test --test models_tests
   ```
   Expected output: Asset creation and bulk operation tests

3. Handler Tests:
   ```bash
   cargo test --test handlers_tests
   ```
   Expected output: API endpoint tests (health check, get assets, buy asset)

4. Performance Tests:
   ```bash
   cargo test --test performance_tests
   ```
   Expected output: Write and read operation metrics

5. Error Handling Tests:
   ```bash
   cargo test --test errors_tests
   ```
   Expected output: Error response validation tests

### Test Output Format

The test output includes:
- Test execution status (success/failure)
- Performance metrics for bulk operations
- Detailed error messages if tests fail

Example output:

running X tests
[bulk_operations] bulk_insert - 150ms - Inserted 1000 assets
[bulk_operations] bulk_read - 50ms - Read 1000 assets
test test_create_new_asset ... ok
test test_bulk_operations ... ok
test test_performance ... ok


## Performance Metrics

Performance tests measure:
- Operations per second
- Average duration per operation
- Total operation count
- Error count

Example metric output:

[performance_test] write_operations - 2500ms - Completed 1000 write operations
Operations/second: 400
Average duration: 2.5ms


## Test Categories

1. **Database Tests** (`db_tests.rs`)
   - Connection pool creation
   - Environment variable handling
   - Error scenarios

2. **Model Tests** (`models_tests.rs`)
   - Single asset creation
   - Bulk asset operations
   - Data validation

3. **Handler Tests** (`handlers_tests.rs`)
   - Health check endpoint
   - Asset listing endpoint
   - Asset purchase endpoint

4. **Performance Tests** (`performance_tests.rs`)
   - Write operation benchmarks
   - Read operation benchmarks
   - Bulk operation metrics

5. **Error Tests** (`errors_tests.rs`)
   - Error response formatting
   - HTTP status code mapping

## Logging

Test logs are stored in:

logs/rust_market_.log

View logs using:

```bash
tail -f logs/rust_market_.log
```


## Common Issues and Solutions

1. **Database Connection Failures**
   - Verify PostgreSQL is running
   - Check DATABASE_URL_TEST in .env.test
   - Ensure test database exists

2. **Permission Issues**
   - Verify database user permissions
   - Check file system permissions for logs

3. **Test Database Reset**
   ```bash
   dropdb rust_market_test
   createdb rust_market_test
   diesel migration run --database-url=postgres://username:password@localhost/rust_market_test
   ```

## Frontend Tests

*Note: Frontend tests will be implemented in future iterations of the project.*

## Continuous Integration

The test suite is designed to run in CI environments. Ensure the following environment variables are set in your CI pipeline:
- DATABASE_URL_TEST
- RUST_LOG

## Contributing New Tests

When adding new tests:
1. Follow the existing pattern in the appropriate test file
2. Include performance metrics for operations tests
3. Add logging statements for debugging
4. Update this guide if adding new test categories

## Test Coverage

To generate test coverage reports:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```


Coverage report will be generated in `target/tarpaulin/`
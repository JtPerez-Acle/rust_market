# rust_market

A high-performance mining equipment marketplace built with Rust, focusing on reliability, security, and scalability.

## Overview

`rust_market` is a specialized marketplace platform for mining equipment, built with Rust. It provides a robust infrastructure for managing equipment listings, orders, and transactions, with a strong emphasis on data integrity and testing.

## Technology Stack

- **Language**: Rust
- **Web Framework**: Actix-Web
- **Database**: PostgreSQL 15
- **ORM**: Diesel 2.2.4
- **Testing**: Rust's built-in testing framework
- **Logging**: Custom logging system using `log` crate
- **Environment Management**: dotenv
- **Connection Pooling**: r2d2

## Features

- **Equipment Management**: Comprehensive system for managing mining equipment listings
- **Order Processing**: Robust order management with transaction support
- **Category System**: Hierarchical equipment categorization
- **User Management**: Secure user authentication and authorization
- **Image Handling**: Support for equipment images and technical documents
- **Maintenance Records**: Track equipment maintenance history
- **Review System**: User reviews and ratings for equipment

## Database Schema

The application uses a well-structured PostgreSQL database with the following key tables:

- `users`: User account information
- `equipment_categories`: Equipment classification
- `equipment`: Mining equipment listings
- `orders`: Purchase orders
- `order_items`: Individual items in orders
- `equipment_images`: Equipment photos and diagrams
- `technical_documents`: Equipment specifications and manuals
- `maintenance_records`: Service history
- `reviews`: User feedback and ratings

## Testing Infrastructure

Our testing infrastructure is designed for reliability and comprehensive coverage:

### Test Environment

- Separate test database configuration via `DATABASE_URL_TEST`
- Isolated test environment for each test run
- Comprehensive cleanup between tests

### Test Categories

1. **Unit Tests**
   - Model validation
   - Data transformation
   - Business logic

2. **Integration Tests**
   - Database operations
   - API endpoints
   - Transaction handling

3. **Performance Tests**
   - Concurrent user operations
   - Database connection pool behavior
   - Transaction isolation

### Test Helpers

Located in `src/test_helpers.rs`, providing:

- Database cleanup with proper foreign key handling
- Transaction management
- Logging setup
- Test data generation

### Database Transaction Management

Our tests use Diesel's transaction API to ensure:

- Atomic operations
- Proper rollback on failure
- Foreign key constraint respect
- Data isolation between tests

## Installation

The project includes an automated setup script that handles all installation steps:

```bash
# 1. Clone the repository
git clone [repository-url]
cd rust_market

# 2. Run the setup script
chmod +x scripts/setup_and_test.sh
./scripts/setup_and_test.sh
```

The setup script will:
- Install required system dependencies (PostgreSQL, Rust)
- Configure the database
- Set up environment variables
- Install Diesel CLI
- Run database migrations
- Build the project
- Run tests

### Requirements
- Linux-based system
- Internet connection for downloading dependencies
- Sudo privileges for installing system packages

### Post-Installation
After successful installation:
1. Run the service: `cargo run`
2. Access the API at: `http://localhost:8080`
3. Run tests: `cargo test`

For detailed technical documentation, see [technical.md](technical.md).

## Project Structure

```
rust_market/
├── src/
│   ├── models/           # Data models
│   ├── schema/          # Database schema
│   ├── handlers/        # Request handlers
│   ├── db/             # Database operations
│   ├── test_helpers.rs # Testing utilities
│   └── main.rs         # Application entry
├── migrations/         # Database migrations
├── tests/             # Integration tests
└── scripts/           # Utility scripts
```

## Error Handling

The application implements comprehensive error handling:

- Custom error types for different scenarios
- Proper error propagation
- Detailed logging
- User-friendly error messages

## Logging

Logging is configured for both development and testing:

- Different log levels (debug, info, error)
- Structured log format
- Separate test logs
- Performance monitoring


## Contributing

1. Fork the repository
2. Create your feature branch
3. Write tests for new features
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Rust community for excellent documentation
- Diesel team for the robust ORM
- Contributors and testers

# Rust Market - Technical Documentation

## Project Overview
This document provides a detailed technical overview of the Rust Market project, an industrial equipment marketplace built using Rust and modern web technologies.

## Architecture Overview

```mermaid
graph TD
    A[Client] -->|HTTP| B[Actix Web Server]
    B --> C[Handlers Layer]
    C --> D[Business Logic]
    D --> E[Database Layer]
    E -->|Diesel ORM| F[(PostgreSQL)]
    
    style A fill:#f9f,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C fill:#ddf,stroke:#333,stroke-width:2px
    style D fill:#ddf,stroke:#333,stroke-width:2px
    style E fill:#ddf,stroke:#333,stroke-width:2px
    style F fill:#bfb,stroke:#333,stroke-width:2px
```

## Project Structure

```mermaid
graph TD
    A[Project Root] --> B[src/]
    A --> C[tests/]
    A --> D[migrations/]
    A --> E[documentation/]
    A --> F[scripts/]
    
    B --> G[handlers/]
    B --> H[db/]
    B --> I[models.rs]
    B --> J[schema.rs]
    B --> K[errors.rs]
    B --> L[logging.rs]
    
    style A fill:#f96,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C fill:#bfb,stroke:#333,stroke-width:2px
    style D fill:#fbb,stroke:#333,stroke-width:2px
```

## Core Components

### 1. Database Layer
- Uses Diesel ORM for database operations
- PostgreSQL as the primary database
- Migrations for schema management
- Models defined in `models.rs`
- Schema definitions in `schema.rs`
- Comprehensive equipment and order management tables
- Support for equipment categories and images
- Maintenance records tracking
- Review system implementation

### 2. Web Server
- Actix-web framework
- RESTful API design
- Async request handling
- Error handling middleware
- JSON payload processing
- File upload handling for equipment images

### 3. Logging System
- Comprehensive logging using `flexi_logger`
- Async logging capabilities
- Structured log format
- Log rotation and management
- Custom log formatting
- Environment-specific logging levels

### 4. Error Handling
- Custom error types using `thiserror`
- Consistent error reporting
- Error conversion traits
- HTTP status code mapping
- Detailed error messages for debugging
- Production-safe error responses

## Dependencies
Key dependencies include:
- actix-web 4.4: Web framework
- diesel 2.1.0: ORM and query builder
- tokio 1.32: Async runtime
- serde 1.0: Serialization framework
- chrono 0.4: DateTime handling
- uuid 1.4: Unique identifier generation
- flexi_logger: Advanced logging capabilities
- bigdecimal: Precise numerical calculations
- jsonb: JSON data type support

## Database Schema

```mermaid
erDiagram
    EQUIPMENT_CATEGORIES ||--o{ EQUIPMENT : categorizes
    EQUIPMENT ||--o{ EQUIPMENT_IMAGES : has
    EQUIPMENT ||--o{ MAINTENANCE_RECORDS : tracks
    EQUIPMENT ||--o{ REVIEWS : receives
    EQUIPMENT ||--o{ ORDER_ITEMS : contains
    ORDERS ||--|{ ORDER_ITEMS : includes
    USERS ||--o{ ORDERS : places
    USERS ||--o{ REVIEWS : writes

    EQUIPMENT {
        int id PK
        int category_id FK
        varchar name
        text description
        varchar manufacturer
        varchar model_number
        int year_manufactured
        varchar condition
        numeric price
        int stock_level
        jsonb specifications
        numeric weight_kg
        varchar dimensions_cm
        varchar power_requirements
        text certification_info
        text warranty_info
        timestamp created_at
        timestamp updated_at
    }

    EQUIPMENT_CATEGORIES {
        int id PK
        varchar name
        text description
        int parent_category_id FK
        timestamp created_at
        timestamp updated_at
    }

    EQUIPMENT_IMAGES {
        int id PK
        int equipment_id FK
        varchar image_url
        bool is_primary
        timestamp created_at
    }

    MAINTENANCE_RECORDS {
        int id PK
        int equipment_id FK
        date service_date
        varchar service_type
        text description
        varchar performed_by
        date next_service_date
        timestamp created_at
    }

    ORDERS {
        int id PK
        int user_id FK
        varchar status
        numeric total_amount
        text shipping_address
        varchar shipping_method
        varchar tracking_number
        date estimated_delivery_date
        text special_instructions
        timestamp created_at
        timestamp updated_at
    }

    ORDER_ITEMS {
        int id PK
        int order_id FK
        int equipment_id FK
        int quantity
        numeric price_at_time
        bool warranty_selected
        text special_requirements
    }

    REVIEWS {
        int id PK
        int equipment_id FK
        int user_id FK
        int rating
        text review_text
        varchar usage_duration
        text pros
        text cons
        timestamp created_at
        timestamp updated_at
    }
```

## Security Considerations
- Password hashing
- Environment variable management
- Database connection pooling
- Input validation
- Error message sanitization
- SQL injection prevention via Diesel ORM
- File upload validation
- Rate limiting implementation
- CORS configuration

## Deployment
- Docker container support
- Environment configuration
- Database migrations
- Logging setup
- Health checks
- Load balancing readiness
- Database backup strategies
- Zero-downtime deployment support

## Performance Optimizations
- Connection pooling
- Async I/O
- Efficient database queries
- Resource management
- Image optimization
- Database indexing
- Query caching strategies
- Bulk operation support

## Monitoring and Logging
- Structured logging
- Error tracking
- Performance metrics
- Audit trails
- Database query monitoring
- Resource usage tracking
- API endpoint metrics
- User activity logging

## Development Environment

```mermaid
graph TD
    A[Development Setup] --> B[Docker Environment]
    B --> C[Application Container]
    B --> D[Database Container]
    
    C --> E[Rust Application]
    C --> F[Development Tools]
    
    D --> G[PostgreSQL]
    D --> H[Init Scripts]
    
    E --> I[Hot Reload]
    E --> J[Debug Mode]
    
    F --> K[Diesel CLI]
    F --> L[Cargo Watch]
    
    style A fill:#f96,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C fill:#bfb,stroke:#333,stroke-width:2px
    style D fill:#fbb,stroke:#333,stroke-width:2px
```

### Docker Configuration
1. **Application Container**
   - Rust development environment
   - Source code mounted for live development
   - Hot reload using cargo-watch
   - Diesel CLI for database management
   - Debug logging enabled
   - Backtrace support

2. **Database Container**
   - PostgreSQL 15
   - Multiple database support (development and test)
   - Persistent volume storage
   - Health check implementation
   - Initialization scripts
   - Exposed on port 5432

### Development Tools
- Cargo Watch: Live code reloading
- Diesel CLI: Database migrations and schema management
- Environment-specific configurations
- Debug logging and backtrace support
- Database initialization scripts
- Development-specific volumes

### Environment Variables
- `DATABASE_URL`: Main database connection
- `DATABASE_URL_TEST`: Test database connection
- `RUST_LOG`: Logging level configuration
- `RUST_BACKTRACE`: Error tracing support
- Database credentials and configuration
- Service ports and networking

### Development Workflow
1. Docker Compose startup
2. Database initialization
3. Migration execution
4. Live code reloading
5. Debug logging
6. Database management

### Best Practices
- Isolated development environment
- Consistent database state
- Live code updates
- Debug-friendly configuration
- Database migration management
- Environment parity with production

## Installation Process

```mermaid
graph TD
    A[Setup Script] --> B[Check Dependencies]
    B --> C[Install PostgreSQL]
    B --> D[Install Rust]
    
    C --> E[Configure Database]
    D --> F[Install Diesel CLI]
    
    E --> G[Run Migrations]
    F --> G
    
    G --> H[Build Project]
    H --> I[Run Tests]
    
    style A fill:#f96,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style E fill:#bfb,stroke:#333,stroke-width:2px
    style G fill:#fbb,stroke:#333,stroke-width:2px
```

### Automated Setup Script
The project includes a comprehensive setup script (`scripts/setup_and_test.sh`) that automates the entire installation process:

1. **System Dependencies Check**
   - PostgreSQL installation verification
   - Rust toolchain setup
   - Required command availability check

2. **Database Setup**
   - PostgreSQL service management
   - Database user configuration
   - Database creation and initialization
   - Connection verification

3. **Environment Configuration**
   - Database URLs setup
   - Logging level configuration
   - Backtrace settings
   - Environment file generation

4. **Project Setup**
   - Diesel CLI installation
   - Database migrations
   - Project build
   - Test execution

### Installation Steps
```bash
# 1. Clone the repository
git clone [repository-url]
cd rust_market

# 2. Run the setup script
chmod +x scripts/setup_and_test.sh
./scripts/setup_and_test.sh
```

### Environment Variables
The setup process configures the following environment variables:
- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_URL_TEST`: Test database connection string
- `RUST_LOG`: Debug logging level
- `RUST_BACKTRACE`: Full backtrace for debugging

### Post-Installation
After successful installation:
1. The service can be run with `cargo run`
2. Tests can be executed with `cargo test`
3. API is accessible at `http://localhost:8080`

### Error Handling
The setup script includes:
- Comprehensive error checking
- Detailed error messages
- Automatic cleanup on failure
- Service verification steps
- Connection testing

### Maintenance
- Database migration management
- Environment configuration updates
- Dependency updates
- Service monitoring
- Log rotation

## Testing Infrastructure

```mermaid
graph TD
    A[Test Suite] --> B[Unit Tests]
    A --> C[Integration Tests]
    A --> D[Performance Tests]
    
    B --> E[Models Tests]
    B --> F[Error Handling Tests]
    
    C --> G[Handler Tests]
    C --> H[Database Tests]
    
    D --> I[Load Testing]
    D --> J[Response Time Tests]
    
    style A fill:#f96,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C fill:#bfb,stroke:#333,stroke-width:2px
    style D fill:#fbb,stroke:#333,stroke-width:2px
```

### Test Components
1. **Unit Tests**
   - `models_tests.rs`: Validates data models and their relationships
   - `errors_tests.rs`: Verifies error handling and custom error types
   - Test isolation using mock data

2. **Integration Tests**
   - `handlers_tests.rs`: Tests API endpoints and request handling
   - `db_tests.rs`: Validates database operations and transactions
   - Docker-based test environment
   - Test database setup and teardown

3. **Performance Tests**
   - `performance_tests.rs`: Load testing and response time benchmarks
   - Concurrent request handling
   - Database query optimization validation

### Test Configuration
- `test_config.rs`: Test environment configuration
- Environment-specific test settings
- Mock data generation
- Test database initialization
- Logging configuration for tests

### CI/CD Integration
- Automated test runs on pull requests
- Test coverage reporting
- Performance benchmark tracking
- Integration with GitHub Actions
- Docker-based test environment

### Test Best Practices
- Isolated test environments
- Comprehensive error case coverage
- Performance regression testing
- Mock external dependencies
- Clean test data management
- Descriptive test naming

## Future Considerations
- Caching layer implementation
- Rate limiting
- API versioning
- Horizontal scaling
- Message queue integration
- Full-text search implementation
- Real-time notifications
- Payment gateway integration
- Inventory management automation

---

This documentation is maintained as part of the project's technical specifications. For updates or contributions, please follow the project's contribution guidelines.
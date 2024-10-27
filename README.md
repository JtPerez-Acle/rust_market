# rust_market

An in-depth look at the `rust_market` project as of October 25, 2024.

## Introduction

`rust_market` is a **high-performance**, **secure**, and **scalable** web application built with Rust. It serves as a real-time marketplace platform that processes information at lightning speed, allowing users to instantly view stock levels, current prices, and product availability. The application leverages Rust's concurrency model, safety guarantees, and modern web frameworks to deliver an ultra-responsive user experience.

## Why Rust?

We chose Rust for several compelling reasons:

- **Performance**: Rust's performance is comparable to C and C++, offering low-level control over system resources without sacrificing speed. This makes it ideal for building high-speed web applications.

- **Safety**: Rust's ownership model ensures memory safety without a garbage collector. It eliminates common bugs such as null pointer dereferencing and data races, which are crucial for a secure marketplace.

- **Concurrency**: Rust provides fearless concurrency, allowing us to write concurrent code that's free from data races. This is essential for handling multiple real-time data streams efficiently.

- **Ecosystem**: Rust has a growing ecosystem with powerful libraries and frameworks like Actix-Web and Diesel, which we utilize extensively in this project.

## Features

- **Real-Time Data Processing**: Instantaneous updates on stock levels, prices, and product availability through efficient data handling and processing.

- **High Performance and Scalability**: Optimized for speed and scalability to handle rapid data changes and a high volume of user requests.

- **Actix-Web Framework**: Utilizes the Actix-Web framework for building fast, reliable, and asynchronous web services.

- **WebSocket Communication**: Implements real-time communication using WebSockets, enabling live updates without the need for client polling.

- **Redis Integration**: Uses Redis for caching frequently accessed data and managing real-time data streams through pub/sub mechanisms.

- **PostgreSQL with Diesel ORM**: Manages data with PostgreSQL, using Diesel ORM for type-safe and efficient database interactions.

- **Blockchain (Web3) Integration Ready**: Structures models and data handling to be compatible with Web3 standards, preparing for seamless integration with blockchain technologies.

- **AI Integration via APIs**: Implements AI functionalities through APIs with robust error handling, leveraging `reqwest` for HTTP requests.

- **Custom Logging System**: Integrates a custom logging system using `flexi_logger` for structured, level-based logging to aid in debugging and monitoring.

- **Comprehensive Testing Suite**: Includes unit tests, integration tests, and performance tests to ensure reliability, correctness, and performance under load.

## Project Milestones

1. **Project Initialization**

   - Initialized the Rust project using `cargo init`.
   - Configured essential dependencies in `Cargo.toml`, including `actix-web`, `diesel`, `dotenv`, `flexi_logger`, and others.

2. **Database Schema Setup**

   - Created database migrations to set up the schema, including tables for `users`, `products`, `orders`, and `order_items`.
   - Ensured data integrity with foreign key constraints and indexed critical fields for performance optimization.

3. **Model Definitions**

   - Defined Rust structs in `src/models.rs` corresponding to the database tables.
   - Derived traits such as `Queryable`, `Insertable`, `Serialize`, and `Deserialize` for efficient database operations and Web3 compatibility.

4. **Database Connection Pool**

   - Established a connection pool in `src/db.rs` using `r2d2` and `diesel`.
   - Implemented robust error handling using `Result` and `Option` types, avoiding `unwrap()` in favor of `expect()` with meaningful messages.

5. **Logging System Implementation**

   - Implemented a custom logging system in `src/logging.rs` using `flexi_logger`.
   - Initialized the logger in `src/main.rs`, setting log levels and formats for comprehensive logging.

6. **Server Configuration**

   - Configured the Actix-Web server with middleware for logging and error handling.
   - Added essential routes and endpoints, including a health check endpoint for monitoring.

7. **WebSocket Server Setup**

   - Implemented a WebSocket server using `tokio-tungstenite` to manage real-time stock updates.
   - Ensured concurrency is handled efficiently using `tokio` for asynchronous operations.

8. **Redis Cache Implementation**

   - Integrated Redis for caching and pub/sub functionalities.
   - Implemented mechanisms to update cache efficiently and invalidate stale data.

9. **Error Handling Enhancements**

   - Improved error handling across the application using the `thiserror` crate.
   - Provided detailed error responses to clients while maintaining security best practices.

10. **Security Enhancements**

    - Enforced input validation and sanitization to prevent common security vulnerabilities.
    - Regularly used `cargo clippy` and `cargo audit` for linting and security checks.
    - Implemented rate limiting and secure authentication mechanisms.

11. **Testing Suite Expansion**

    - Expanded tests to include unit tests for individual modules, integration tests for API endpoints, and performance tests under load.
    - Used `actix-web`'s testing utilities and a test database instance to simulate real-world scenarios.

## Architecture Overview

```mermaid
flowchart TD
    subgraph Client Side
        A[Web Browser]
    end
    subgraph Server Side
        B[Actix-Web & WebSocket Server]
        C[Handlers]
        D[Real-Time Processor]
        E[Database Connection Pool]
        F[(PostgreSQL Database)]
        G[(Redis Cache)]
    end
    A <-->|HTTP/WebSocket| B
    B --> C
    C --> D
    D --> E
    E --> F
    D --> G
```

## How It Works

### Real-Time Stock Updates

- **Data Flow**:

  1. Inventory updates are sent to the server via API calls.
  2. Server updates the PostgreSQL database and Redis cache.
  3. Redis publishes the update through its pub/sub mechanism.
  4. WebSocket server receives the message and broadcasts it to connected clients.
  5. Clients receive the update and refresh the stock information in real-time.

- **Concurrency Management**:

  - Utilizes `tokio` for asynchronous tasks, ensuring that multiple requests are handled efficiently without blocking the main thread.

### Web3 Integration

- **Blockchain-Ready Models**:

  - Structs are designed to be compatible with Web3 standards, deriving `Serialize` and `Deserialize` traits for seamless JSON serialization.

- **Smart Contract Interactions**:

  - The application is prepared for future integration with smart contracts, ensuring that data structures align with blockchain requirements.

- **Security and Validation**:

  - Inputs are rigorously validated to prevent malicious data from affecting blockchain transactions.

### AI Integration via APIs

- **External API Calls**:

  - Uses `reqwest` for making HTTP requests to AI services.
  - Implements strong error handling with retries and meaningful error messages.

- **JSON Parsing**:

  - Parses AI responses carefully to prevent runtime errors and ensure data integrity.
  - Validates and sanitizes AI-generated content before using it in the application.

- **Rate Limiting and Logging**:

  - Manages request frequency to adhere to API rate limits.
  - Logs API interactions for monitoring and debugging purposes.

## Getting Started

### Prerequisites

- **Rust**: Ensure Rust is installed on your system.
- **PostgreSQL**: Set up a PostgreSQL database instance.
- **Redis**: Install Redis for caching and real-time data processing.
- **Diesel CLI**: Install Diesel CLI for managing database migrations.

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/JtPerez-Acle/rust_market.git
   cd rust_market
   ```

2. **Set Up Environment Variables**:

   Create a `.env` file and add your database and Redis URLs:

   ```env
   DATABASE_URL=postgres://username:password@localhost/rust_market
   REDIS_URL=redis://localhost:6379
   ```

3. **Run Database Migrations**:

   ```bash
   diesel migration run
   ```

4. **Build and Run the Application**:

   ```bash
   cargo run
   ```

### Testing

Run the test suite using:

```bash
cargo test
```

To run performance tests:

```bash
cargo test --test performance_tests -- --nocapture
```

## Documentation

Detailed documentation is available in the `documentation` directory, including:

- **Core Documentation**: `documentation/core.md` provides an overview of the project's architecture and components.
- **Testing Guide**: `tests.md` outlines how to set up and run tests.
- **Development Progress**: `current.md` and `documentation/25102024.md` contain information about current development status and next steps.

## Contributing

Contributions are welcome! Please see `CONTRIBUTING.md` for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License.

# rust_market

An overview of the `rust_market` project as of October 25, 2024.

## Introduction

`rust_market` is a **high-performance** Rust-based web application designed to provide a **real-time marketplace platform**. The main idea of the project is to **process information super fast**, enabling users to view stock, current prices, and availability **instantly**. The project leverages robust frameworks and libraries to ensure **ultra-fast processing**, security, and scalability.

## Features

- **Real-Time Data Processing**: Provides instantaneous updates on stock levels, prices, and product availability.
- **High Performance**: Optimized for speed to handle rapid data changes and user requests efficiently.
- **Actix-Web Server**: Utilizes the Actix-Web framework for building fast and reliable web services.
- **PostgreSQL with Diesel ORM**: Implements PostgreSQL as the database, managed through Diesel ORM for type-safe and efficient database interactions.
- **Blockchain Integration**: Prepares models and structures compatible with Web3 standards, ensuring seamless interaction with blockchain technologies.
- **AI Integration via APIs**: Plans to integrate AI functionalities through APIs, with strong error handling and logging mechanisms using `reqwest`.
- **Custom Logging System**: Integrated a custom logging system using `flexi_logger` for structured and level-based logging.

## Project Milestones

- **Project Initialization**: Set up with `cargo init` and configured with essential dependencies such as `actix-web`, `diesel`, `dotenv`, and others.
- **Database Schema Setup**: Created migrations for setting up the database schema, including tables for `users`, `products`, `orders`, and `order_items`.
- **Model Definitions**: Defined Rust structs in `src/models.rs` corresponding to database tables, deriving traits like `Queryable`, `Insertable`, `Serialize`, and `Deserialize` for Web3 compatibility.
- **Database Connection Pool**: Established a connection pool in `src/db.rs` using `r2d2` and `diesel`, with robust error handling and environment variable management.
- **Logging System Implementation**: Implemented a custom logging system in `src/logging.rs` using `flexi_logger`, and initialized it in `src/main.rs`.
- **Server Configuration**: Configured the Actix-Web server with middleware for logging and included a health check endpoint.
- **Testing Suite**: Implemented comprehensive tests to ensure reliability and correctness of application components.

## Architecture Overview

```mermaid
flowchart TD
    A[Client] --> B[Actix-Web Server]
    B --> C[Handlers]
    C --> D[Real-Time Processor]
    D --> E[Database Connection Pool]
    E --> F[PostgreSQL Database]
```

## Getting Started

### Prerequisites

- **Rust**: Ensure Rust is installed on your system.
- **PostgreSQL**: Set up a PostgreSQL database instance.
- **Diesel CLI**: Install Diesel CLI for managing database migrations.

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/JtPerez-Acle/rust_market.git
   cd rust_market
   ```

2. **Set Up Environment Variables**:

   Create a `.env` file and add your database URL:

   ```env
   DATABASE_URL=postgres://username:password@localhost/rust_market
   ```

3. **Run Database Migrations**:

   ```bash
   diesel migration run
   ```

4. **Build and Run the Application**:

   ```bash
   cargo run
   ```

## Testing

Run the test suite using:

```bash
cargo test
```

## License

This project is licensed under the MIT License.

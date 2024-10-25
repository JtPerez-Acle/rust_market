# Core Documentation

This document provides an overview of the rust_market project, detailing its core components, architecture, and workflows. It includes diagrams to visualize different parts of the system using Mermaid.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
   - [High-Level System Architecture](#high-level-system-architecture)
   - [Backend Components](#backend-components)
   - [Frontend Components](#frontend-components)
3. [Data Flow](#data-flow)
   - [Real-Time Stock Updates Workflow](#real-time-stock-updates-workflow)
   - [Order Processing Workflow](#order-processing-workflow)
4. [Technologies Used](#technologies-used)
5. [Directory Structure](#directory-structure)
6. [Future Enhancements](#future-enhancements)

## Project Overview

The rust_market project is an e-commerce marketplace developed in Rust. The primary goal is to create a platform where stock levels and prices are updated in real-time, ensuring users always see the latest information. This is achieved through efficient backend services and real-time communication protocols.

## Architecture

### High-Level System Architecture

The system follows a client-server architecture with real-time capabilities. For the detailed diagram, please refer to `documentation/diagrams/system_architecture.md`.

### Backend Components

- **API Server**: Handles RESTful API requests for standard operations like user authentication, product listing, and order placement.
- **WebSocket Server**: Manages real-time communication for stock and price updates.
- **Database**: PostgreSQL database for persistent storage of users, products, orders, etc.
- **Cache**: Redis is used for caching frequently accessed data and managing real-time data streams.

### Frontend Components

- **Web Application**: Built using React.js, it provides the user interface for customers and sellers.
- **WebSocket Client**: Embedded in the web application to receive real-time updates from the server.

## Data Flow

### Real-Time Stock Updates Workflow

This workflow illustrates how stock updates propagate from the inventory management system to the user's browser in real-time. For the detailed diagram, please refer to `documentation/diagrams/stock_updates_workflow.md`.

### Order Processing Workflow

This workflow shows the steps involved when a user places an order. For the detailed diagram, please refer to `documentation/diagrams/order_processing_workflow.md`.

## Technologies Used

- **Programming Language**: Rust
- **Backend Framework**: Actix-web
- **Asynchronous Runtime**: Tokio
- **Database**: PostgreSQL
- **Object-Relational Mapping (ORM)**: Diesel
- **Cache**: Redis
- **WebSocket Library**: tokio-tungstenite
- **Frontend Framework**: React.js
- **Communication Protocols**: RESTful APIs, WebSockets
- **Containerization**: Docker
- **Version Control**: Git

## Directory Structure

```
rust_market/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── api/
│   ├── websocket/
│   ├── models/
│   ├── schema.rs
│   └── utils/
├── static/
│   └── index.html
├── documentation/
│   ├── core.md
│   └── diagrams/
├── frontend/
│   ├── package.json
│   └── src/
└── docker-compose.yml
```

## Future Enhancements

1. **Scalability Improvements**
   - Implement load balancing.
   - Use Kubernetes for orchestration.

2. **Microservices Architecture**
   - Break down the monolithic application into microservices.

3. **Additional Features**
   - Implement user reviews and ratings.
   - Add support for multiple payment gateways.

4. **Security Enhancements**
   - Implement two-factor authentication.
   - Conduct regular security audits.
   - Use secure WebSocket connections (WSS).

5. **Performance Optimization**
   - Implement database query optimization.
   - Use content delivery networks (CDNs) for static assets.

6. **Analytics and Monitoring**
   - Integrate logging and monitoring tools.
   - Implement real-time analytics for business insights.

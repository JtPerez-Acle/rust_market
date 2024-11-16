# Build stage
FROM rust:latest as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build app with release profile
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install OpenSSL and CA certificates for HTTPS
RUN apt-get update && \
    apt-get install -y openssl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/rust_market /usr/local/bin/
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
COPY --from=builder /usr/src/app/diesel.toml /usr/local/bin/

# Install diesel_cli for migrations
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /usr/local/bin

# Set environment variables that Heroku will override
ENV DATABASE_URL=postgres://postgres:postgres@db/rust_market
ENV PORT=8080

# Create script to run migrations and start the app
RUN echo '#!/bin/bash\n\
diesel migration run\n\
exec rust_market' > /usr/local/bin/start.sh && \
chmod +x /usr/local/bin/start.sh

EXPOSE 8080

CMD ["./start.sh"]

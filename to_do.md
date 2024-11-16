# Dockerization Plan for Rust Market

## 1. Project Structure Setup
- Create a multi-stage Dockerfile for optimal image size
- Setup .dockerignore file to exclude unnecessary files
- Create docker-compose.yml for local development

## 2. Docker Configuration Files

### 2.1 Create Dockerfile
```dockerfile
# Build stage
FROM rust:1.73 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/rust_market .
COPY --from=builder /usr/src/app/.env .
EXPOSE 8080
CMD ["./rust_market"]
```

### 2.2 Create .dockerignore
```
target/
.git/
.gitignore
.env.test
tests/
documentation/
*.md
!README.md
```

### 2.3 Create docker-compose.yml
```yaml
version: '3.8'
services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://postgres:postgres@db/rust_market
      - RUST_LOG=debug
    depends_on:
      - db

  db:
    image: postgres:15
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=rust_market
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## 3. Implementation Steps

1. **Database Configuration**
   - Update database connection settings for containerized environment
   - Ensure migrations run automatically on startup
   - Setup proper database volume management

2. **Environment Variables**
   - Move sensitive data to environment variables
   - Create separate .env files for different environments
   - Document all required environment variables

3. **Build and Test Process**
   - Create build scripts for Docker
   - Setup CI/CD pipeline configuration
   - Implement health checks

4. **Production Considerations**
   - Implement proper logging configuration
   - Setup monitoring and metrics
   - Configure proper security settings
   - Implement backup strategy for database

5. **Documentation Updates**
   - Update README.md with Docker instructions
   - Document deployment process
   - Add troubleshooting guide

## 4. Testing Strategy

1. **Local Testing**
   - Test building Docker image
   - Test docker-compose setup
   - Verify all services connect properly

2. **Integration Testing**
   - Test database migrations
   - Verify environment variables
   - Check logging functionality

3. **Performance Testing**
   - Test application under load
   - Monitor resource usage
   - Verify container scaling

## 5. Deployment Checklist

- [ ] Create production-ready Dockerfile
- [ ] Setup proper environment variables
- [ ] Configure database connections
- [ ] Test all endpoints
- [ ] Setup monitoring
- [ ] Configure logging
- [ ] Setup backup strategy
- [ ] Document deployment process
- [ ] Create deployment scripts
- [ ] Test scaling configuration

## 6. Next Steps

1. Implement the Dockerfile
2. Create the docker-compose.yml file
3. Update the database configuration
4. Test the containerized application
5. Document the process in README.md

## 7. Immediate Action - Test Environment Setup
### Running Tests with Docker PostgreSQL

1. Create a docker-compose.test.yml:
```yaml
version: '3.8'
services:
  db_test:
    image: postgres:15
    environment:
      - POSTGRES_USER=jtdev
      - POSTGRES_PASSWORD=dev1998
      - POSTGRES_DB=rust_market_test
    ports:
      - "5432:5432"
```

2. Steps to run tests:
   - Start test database container
   - Run database migrations
   - Execute test suite

### Commands to run tests:
```bash
# Start the test database
docker-compose -f docker-compose.test.yml up -d

# Wait for database to be ready (about 5-10 seconds)
sleep 10

# Run the tests
cargo test
```

## 8. Docker Development Workflow

### Local Development
1. Start the development environment:
```bash
docker-compose up --build
```
This will:
- Build the Rust application
- Start PostgreSQL database
- Enable hot-reloading with cargo-watch
- Mount your source code for live development

2. Access the application:
- API: http://localhost:8080
- Database: localhost:5432 (credentials in docker-compose.yml)

### Running Tests
1. Run the test suite:
```bash
docker-compose -f docker-compose.test.yml up --build --abort-on-container-exit
```
This will:
- Build the test environment
- Start a separate test database
- Run all tests
- Exit when tests complete

### Production Deployment (Heroku)
1. Install Heroku CLI and login
2. Set up Heroku PostgreSQL addon
3. Deploy using:
```bash
heroku container:push web
heroku container:release web
```

### Environment Variables
- Development: Set in docker-compose.yml
- Testing: Set in docker-compose.test.yml
- Production: Set through Heroku dashboard or CLI

### Database Migrations
Migrations run automatically:
- On development: When container starts
- On testing: Before tests run
- On production: Before application starts

### Troubleshooting
1. Database connection issues:
   - Check if database container is running: `docker-compose ps`
   - View logs: `docker-compose logs db`
   
2. Application issues:
   - View logs: `docker-compose logs app`
   - Access container: `docker-compose exec app bash`

3. Test issues:
   - View test logs: `docker-compose -f docker-compose.test.yml logs`
   - Run specific tests: `docker-compose -f docker-compose.test.yml run app_test cargo test test_name`
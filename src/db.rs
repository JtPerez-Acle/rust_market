use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use log::error;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the PostgreSQL database
/// 
/// # Returns
/// - `Ok(DbPool)` if the connection pool is successfully created
/// - `Err(PoolError)` if there's an error creating the pool or reading environment variables
pub fn establish_connection_pool() -> Result<DbPool, r2d2::PoolError> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get database URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");

    // Create a connection manager
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Build and return the connection pool
    r2d2::Pool::builder()
        .build(manager)
        .map_err(|e| {
            error!("Failed to create database connection pool: {}", e);
            e
        })
}

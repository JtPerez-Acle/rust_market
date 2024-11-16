use dotenv::dotenv;
use std::env;
use crate::logging;
use log::{info, error};
use std::fs;
use chrono::Utc;
use std::sync::Once;
use diesel::RunQueryDsl;
use crate::schema::{users::dsl::*, orders::dsl::*, order_items::dsl::*, products::dsl::*};
use crate::db;

// Used to ensure logger is initialized only once
static INIT: Once = Once::new();

pub fn setup() {
    // Create logs directory if it doesn't exist
    fs::create_dir_all("logs").expect("Failed to create logs directory");

    // Try to load .env.test first, fall back to .env if not found
    if dotenv::from_filename(".env.test").is_err() {
        dotenv().ok();
    }

    // Set test-specific environment variables if not already set
    if env::var("RUST_LOG").is_err() {
        // Keep the RUST_LOG environment variable as is
        env::set_var("RUST_LOG", "debug,r2d2=warn");
    }

    // Initialize logger only once
    INIT.call_once(|| {
        if let Err(e) = logging::init_logger() {
            panic!("Critical Error: Failed to initialize test logger: {}", e);
        }
    });

    // Log test execution
    info!("Running test setup at {}", Utc::now());
}

pub fn cleanup_database(pool: &db::DbPool) {
    let conn = &mut pool.get().expect("Failed to get db connection");
    info!("Starting database cleanup at {}", Utc::now());
    
    // Use a transaction to ensure atomicity of cleanup operations
    conn.build_transaction()
        .read_write()
        .run(|conn| {
            // Delete in order of dependencies to avoid foreign key violations
            match diesel::delete(order_items).execute(conn) {
                Ok(count) => info!("Deleted {} records from order_items", count),
                Err(e) => error!("Failed to clean up order_items table: {}", e),
            }
                
            match diesel::delete(orders).execute(conn) {
                Ok(count) => info!("Deleted {} records from orders", count),
                Err(e) => error!("Failed to clean up orders table: {}", e),
            }
                
            match diesel::delete(products).execute(conn) {
                Ok(count) => info!("Deleted {} records from products", count),
                Err(e) => error!("Failed to clean up products table: {}", e),
            }
                
            match diesel::delete(users).execute(conn) {
                Ok(count) => info!("Deleted {} records from users", count),
                Err(e) => error!("Failed to clean up users table: {}", e),
            }
            
            Ok::<_, diesel::result::Error>(())
        })
        .expect("Failed to execute cleanup transaction");
    
    info!("Completed database cleanup at {}", Utc::now());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        setup();
        // Verify logs directory exists
        assert!(fs::metadata("logs").is_ok(), "Logs directory should exist");
    }
}

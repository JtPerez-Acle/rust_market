use dotenv::dotenv;
use std::env;
use crate::logging;
use log::{info, error};
use std::fs;
use chrono::Utc;
use std::sync::Once;
use diesel::RunQueryDsl;
use diesel::result::Error as DieselError;
use crate::schema::{users::dsl::*, orders::dsl::*, order_items::dsl::*, equipment::dsl::*, equipment_categories::dsl::*, equipment_images::dsl::*, reviews::dsl::*, maintenance_records::dsl::*, technical_documents::dsl::*};
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
        .run::<_, DieselError, _>(|conn| {
            // Delete in order of dependencies to avoid foreign key violations
            // First delete tables that have foreign keys to other tables
            match diesel::delete(technical_documents).execute(conn) {
                Ok(count) => info!("Deleted {} records from technical_documents", count),
                Err(e) => error!("Error deleting technical_documents: {}", e),
            }

            match diesel::delete(maintenance_records).execute(conn) {
                Ok(count) => info!("Deleted {} records from maintenance_records", count),
                Err(e) => error!("Error deleting maintenance_records: {}", e),
            }

            match diesel::delete(reviews).execute(conn) {
                Ok(count) => info!("Deleted {} records from reviews", count),
                Err(e) => error!("Error deleting reviews: {}", e),
            }

            match diesel::delete(equipment_images).execute(conn) {
                Ok(count) => info!("Deleted {} records from equipment_images", count),
                Err(e) => error!("Error deleting equipment_images: {}", e),
            }

            match diesel::delete(order_items).execute(conn) {
                Ok(count) => info!("Deleted {} records from order_items", count),
                Err(e) => error!("Error deleting order_items: {}", e),
            }

            match diesel::delete(orders).execute(conn) {
                Ok(count) => info!("Deleted {} records from orders", count),
                Err(e) => error!("Error deleting orders: {}", e),
            }

            match diesel::delete(equipment).execute(conn) {
                Ok(count) => info!("Deleted {} records from equipment", count),
                Err(e) => error!("Error deleting equipment: {}", e),
            }

            match diesel::delete(equipment_categories).execute(conn) {
                Ok(count) => info!("Deleted {} records from equipment_categories", count),
                Err(e) => error!("Error deleting equipment_categories: {}", e),
            }

            // Finally delete users, which other tables depend on
            match diesel::delete(users).execute(conn) {
                Ok(count) => info!("Deleted {} records from users", count),
                Err(e) => error!("Error deleting users: {}", e),
            }

            Ok(())
        })
        .expect("Failed to clean up database");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        setup();
        assert!(env::var("RUST_LOG").is_ok());
    }
}

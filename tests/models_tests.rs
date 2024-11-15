use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;
use std::time::Instant;
use log::{info, error};
use rust_market::models::{Asset, NewAsset};
use rust_market::schema::assets;
use diesel::Connection;
use std::sync::Once;

// Use the common module from rust_market
use rust_market::common::setup_test_db;
use rust_market::common::test_logging::{log_test_metric, TestMetric, calculate_performance_metrics};

static INIT: Once = Once::new();

// Helper Functions
fn generate_unique_identifier() -> String {
    Uuid::new_v4().to_string().split('-').next().unwrap().to_string()
}

fn insert_bulk_assets(connection: &mut PgConnection, num_assets: usize) {
    let unique_id = generate_unique_identifier();
    let new_assets: Vec<NewAsset> = (0..num_assets).map(|i| {
        NewAsset {
            name: format!("Asset {} {}", unique_id, i),
            price: BigDecimal::from_str("9.99").expect("Invalid price"),
            stock: 100,
            image_url: format!("https://example.com/asset{}.jpg", i),
        }
    }).collect();

    diesel::insert_into(assets::table)
        .values(&new_assets)
        .execute(connection)
        .expect("Failed to insert assets");
}

// Tests Section
#[test]
fn test_create_new_asset() {
    initialize();
    let pool = setup_test_db();
    let mut connection = pool.get().expect("Failed to get connection from pool");

    let unique_id = generate_unique_identifier();

    let new_asset = NewAsset {
        name: format!("Test Asset {}", unique_id),
        price: BigDecimal::from_str("19.99").expect("Invalid price"),
        stock: 100,
        image_url: format!("https://example.com/asset{}.jpg", unique_id),
    };

    let result = diesel::insert_into(assets::table)
        .values(&new_asset)
        .get_result::<Asset>(&mut connection);

    match result {
        Ok(asset) => {
            assert_eq!(asset.name, new_asset.name);
            assert_eq!(asset.price, new_asset.price);
            assert_eq!(asset.stock, new_asset.stock);
        },
        Err(e) => panic!("Failed to insert new asset: {:?}", e),
    }
}

#[test]
fn test_bulk_operations() {
    initialize();
    let pool = setup_test_db();
    let mut connection = pool.get().expect("Failed to get connection from pool");
    let num_assets = 1000;
    let mut error_count = 0;

    info!("Starting bulk operations test");

    // Test bulk insert
    let start = Instant::now();
    insert_bulk_assets(&mut connection, num_assets);
    let duration = start.elapsed();
    
    log_test_metric(TestMetric::new(
        "bulk_operations",
        "bulk_insert",
        duration,
        true,
        Some(format!("Inserted {} assets", num_assets)),
        Some(calculate_performance_metrics(num_assets, duration, error_count)),
    ));

    // Test bulk read
    let read_start = Instant::now();
    match assets::table.load::<Asset>(&mut connection) {
        Ok(assets) => {
            let read_duration = read_start.elapsed();
            log_test_metric(TestMetric::new(
                "bulk_operations",
                "bulk_read",
                read_duration,
                true,
                Some(format!("Read {} assets", assets.len())),
                Some(calculate_performance_metrics(assets.len(), read_duration, error_count)),
            ));
            assert!(assets.len() >= num_assets, "Should have at least {} assets", num_assets);
        },
        Err(e) => {
            error_count += 1;
            log_test_metric(TestMetric::new(
                "bulk_operations",
                "bulk_read",
                read_start.elapsed(),
                false,
                Some(format!("Failed to read assets: {}", e)),
                None,
            ));
            panic!("Bulk read failed: {}", e);
        }
    }
}

fn initialize() {
    INIT.call_once(|| {
        env_logger::init();
    });
}


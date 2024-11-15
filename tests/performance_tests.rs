// tests/performance_tests.rs

use std::time::{Duration, Instant};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use log::{error, info};
use std::env;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::sync::Once;

use rust_market::models::{Asset, NewAsset};
use rust_market::schema::assets;

// Use the common module from rust_market
use rust_market::common::test_logging::{log_test_metric, TestMetric, calculate_performance_metrics};
use rust_market::common::setup_test_db;

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[test]
fn test_performance() -> Result<(), Box<dyn std::error::Error>> {
    initialize();

    let pool = setup_test_db();
    let mut connection = pool.get().expect("Failed to get connection from pool");

    // Run performance tests
    run_performance_tests(&mut connection)?;

    Ok(())
}

fn run_performance_tests(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    let iterations = 1000;
    let mut write_errors = 0;
    let mut read_errors = 0;
    let start_time = Instant::now();

    // Write Performance Test
    let write_start = Instant::now();
    for i in 0..iterations {
        let new_asset = NewAsset {
            name: format!("Asset {}", i),
            price: BigDecimal::from_str("9.99").expect("Invalid price"),
            stock: 100,
            image_url: format!("https://example.com/asset{}.jpg", i),
        };

        if let Err(e) = diesel::insert_into(assets::table)
            .values(&new_asset)
            .execute(conn)
        {
            write_errors += 1;
            error!("Failed to insert new asset: {:?}", e);
        }
    }
    let write_duration = write_start.elapsed();

    // Log write performance
    log_test_metric(TestMetric::new(
        "performance_test",
        "write_operations",
        write_duration,
        write_errors == 0,
        Some(format!("Completed {} write operations", iterations)),
        Some(calculate_performance_metrics(iterations, write_duration, write_errors)),
    ));

    // Read Performance Test
    let read_start = Instant::now();
    for _ in 0..iterations {
        if let Err(e) = assets::table.limit(100).load::<Asset>(conn) {
            read_errors += 1;
            error!("Failed to load assets: {:?}", e);
        }
    }
    let read_duration = read_start.elapsed();

    // Log read performance
    log_test_metric(TestMetric::new(
        "performance_test",
        "read_operations",
        read_duration,
        read_errors == 0,
        Some(format!("Completed {} read operations", iterations)),
        Some(calculate_performance_metrics(iterations, read_duration, read_errors)),
    ));

    // Log overall test performance
    log_test_metric(TestMetric::new(
        "performance_test",
        "overall",
        start_time.elapsed(),
        write_errors == 0 && read_errors == 0,
        Some(format!(
            "Completed {} total operations with {} errors",
            iterations * 2,
            write_errors + read_errors
        )),
        Some(calculate_performance_metrics(
            iterations * 2,
            start_time.elapsed(),
            write_errors + read_errors,
        )),
    ));

    Ok(())
}

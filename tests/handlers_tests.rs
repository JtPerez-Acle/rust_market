use actix_web::{test, http::StatusCode, App, web};
use rust_market::handlers::{self, health_check};
use rust_market::common::setup_test_db;
use rust_market::db::Pool;
use serde_json::json;
use std::sync::Once;
use log::info;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::connection::Connection;
use diesel::sql_types::Bool;
use diesel::deserialize::QueryableByName;

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        std::env::set_var("RUST_TEST", "1");
        rust_market::logging::init_logger().expect("Failed to initialize logger");
    });
}

async fn setup_test_environment() -> Pool {
    initialize();
    let pool = setup_test_db();
    
    // Run migrations
    let mut conn = pool.get().expect("Failed to get DB connection");
    conn.begin_test_transaction().expect("Failed to start test transaction");
    
    pool
}

// Add helper function for future database setup needs
async fn setup_database_schema(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Create a transaction for all schema changes
    conn.transaction(|conn| {
        // Split and execute each command
        let migration_sql = include_str!("../migrations/2024-10-25-171428_create_market_tables/up.sql");
        for command in migration_sql.split(';').filter(|s| !s.trim().is_empty()) {
            diesel::sql_query(command)
                .execute(conn)?;
        }
        Ok(())
    })
}

// Add helper function for database cleanup
async fn teardown_database(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get()?;
    
    // Drop all tables in the correct order
    conn.transaction(|conn| {
        let tables = ["assets", "order_items", "orders", "products", "users"];
        for table in tables.iter() {
            if let Ok(true) = check_table_exists(conn, table) {
                diesel::sql_query(format!("DROP TABLE IF EXISTS {} CASCADE", table))
                    .execute(conn)?;
            }
        }
        Ok(())
    })
}

// Helper function to check if a table exists
fn check_table_exists(conn: &mut PgConnection, table_name: &str) -> Result<bool, diesel::result::Error> {
    #[derive(QueryableByName)]
    struct TableExists {
        #[diesel(sql_type = Bool)]
        exists: bool,
    }

    let result: TableExists = diesel::sql_query(format!(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_name = '{}'
        ) as exists",
        table_name
    ))
    .get_result(conn)?;

    Ok(result.exists)
}

#[actix_web::test]
async fn test_health_check() {
    setup_test_environment().await;

    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(
        body,
        r#"{"status":"healthy"}"#.as_bytes(),
        "Unexpected response body"
    );
}

#[actix_web::test]
async fn test_get_assets() {
    let pool = setup_test_environment().await;
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Run migrations first
    rust_market::migrations::run_migrations(&mut conn)
        .expect("Failed to run migrations");

    // Insert test assets into the database
    {
        use rust_market::models::NewAsset;
        use rust_market::schema::assets::dsl::*;
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        let new_asset = NewAsset {
            name: "Test Asset".to_string(),
            price: BigDecimal::from_str("9.99").unwrap(),
            stock: 100,
            image_url: "https://example.com/asset.jpg".to_string(),
        };

        // Start a transaction for the test
        conn.transaction(|conn| {
            diesel::insert_into(assets)
                .values(&new_asset)
                .execute(conn)
        }).expect("Failed to insert test asset");
    }

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_assets)
    ).await;

    let req = test::TestRequest::get().uri("/api/assets").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK, "Response status should be 200 OK");

    // Add response body check
    let body = test::read_body(resp).await;
    let assets: Vec<serde_json::Value> = serde_json::from_slice(&body)
        .expect("Response should be valid JSON array");
    assert!(!assets.is_empty(), "Response should contain at least one asset");
}

#[actix_web::test]
async fn test_buy_asset() {
    let pool = setup_test_environment().await;
    let mut conn = pool.get().expect("Failed to get DB connection");

    // Run migrations first
    rust_market::migrations::run_migrations(&mut conn)
        .expect("Failed to run migrations");

    // Insert a test asset and get its ID
    let asset_id = {
        use rust_market::models::NewAsset;
        use rust_market::schema::assets::dsl::*;
        use bigdecimal::BigDecimal;
        use std::str::FromStr;

        let new_asset = NewAsset {
            name: "Test Asset".to_string(),
            price: BigDecimal::from_str("9.99").unwrap(),
            stock: 100,
            image_url: "https://example.com/asset.jpg".to_string(),
        };

        // Start a transaction for the test
        conn.transaction(|conn| {
            diesel::insert_into(assets)
                .values(&new_asset)
                .returning(id)
                .get_result::<i32>(conn)
        }).expect("Failed to insert test asset")
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::buy_asset)
    ).await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/assets/{}/buy", asset_id))
        .set_json(&json!({
            "quantity": 1
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success(), 
        "Response status should be successful, got: {} with body: {:?}", 
        resp.status(),
        test::read_body(resp).await
    );
}

#[path = "test_logging.rs"]
mod test_logging;
#[path = "test_config.rs"]
mod test_config;

use test_logging::TestLogger;
use test_config::setup_test_db;
use rust_market::models::{NewUser, User, NewProduct, Product, NewOrder};
use rust_market::schema::{users, products, orders};
use diesel::prelude::*;
use rust_market::db::establish_connection_pool;
use uuid::Uuid;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use futures::future::join_all;
use std::time::Instant;
use bigdecimal::BigDecimal;
use std::str::FromStr;

type BoxError = Box<dyn Error + Send + Sync + 'static>;

fn setup() {
    setup_test_db();
}

async fn setup_test_user(conn: &mut PgConnection) -> Result<User, BoxError> {
    let new_user = NewUser::new(
        format!("test_user_{}", Uuid::new_v4()),
        format!("test{}@example.com", Uuid::new_v4()),
        "test_hash".to_string(),
    );

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn)
        .map_err(|e| Box::new(e) as BoxError)
}

async fn setup_test_product(conn: &mut PgConnection) -> Result<Product, BoxError> {
    let new_product = NewProduct::new(
        format!("test_product_{}", Uuid::new_v4()),
        Some("Test product".to_string()),
        BigDecimal::from_str("10.00").unwrap(),
        100,
    );

    diesel::insert_into(products::table)
        .values(&new_product)
        .get_result::<Product>(conn)
        .map_err(|e| Box::new(e) as BoxError)
}

#[tokio::test]
async fn test_order_limits() -> Result<(), BoxError> {
    setup();
    let logger = Arc::new(Mutex::new(TestLogger::new(
        "order_limits",
        "Test order processing limits and constraints"
    )));
    let pool = establish_connection_pool(None)?;
    let conn = &mut pool.get()?;

    // Setup test data
    let user = setup_test_user(conn).await?;
    let product = setup_test_product(conn).await?;

    // Test 1: Order with zero quantity
    let start = Instant::now();
    let result = create_test_order(conn, user.id, product.id, 0).await;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("zero_quantity_order", duration);
        if let Err(e) = &result {
            logger.log_error("zero_quantity_order", &e.to_string());
        }
    }

    // Test 2: Order with negative quantity
    let start = Instant::now();
    let result = create_test_order(conn, user.id, product.id, -1).await;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("negative_quantity_order", duration);
        if let Err(e) = &result {
            logger.log_error("negative_quantity_order", &e.to_string());
        }
    }

    // Test 3: Order exceeding available stock
    let start = Instant::now();
    let result = create_test_order(conn, user.id, product.id, 101).await;
    let duration = start.elapsed();
    {
        let mut logger = logger.lock().unwrap();
        logger.log_operation("excess_stock_order", duration);
        if let Err(e) = &result {
            logger.log_error("excess_stock_order", &e.to_string());
        }
    }

    // Test 4: Multiple concurrent orders
    let mut handles = vec![];
    let num_orders = 10;
    let quantity_per_order = 10;

    for _ in 0..num_orders {
        let pool = pool.clone();
        let logger = Arc::clone(&logger);
        let user_id = user.id;
        let product_id = product.id;

        handles.push(tokio::spawn(async move {
            let conn = &mut pool.get()?;
            let start = Instant::now();
            let result = create_test_order(conn, user_id, product_id, quantity_per_order).await;
            let duration = start.elapsed();
            
            {
                let mut logger = logger.lock().unwrap();
                logger.log_operation("concurrent_order", duration);
                if let Err(e) = &result {
                    logger.log_error("concurrent_order", &e.to_string());
                }
            }
            
            result
        }));
    }

    let results = join_all(handles).await;
    for result in results {
        match result {
            Ok(Ok(())) => (),
            Ok(Err(e)) => {
                {
                    let mut logger = logger.lock().unwrap();
                    logger.log_error("concurrent_orders", &e.to_string());
                }
                return Err(e);
            }
            Err(e) => {
                {
                    let mut logger = logger.lock().unwrap();
                    logger.log_error("concurrent_orders", &e.to_string());
                }
                return Err(Box::new(e));
            }
        }
    }

    {
        let mut logger = logger.lock().unwrap();
        logger.finish(true, None);
    }
    Ok(())
}

async fn create_test_order(
    conn: &mut PgConnection,
    user_id: i32,
    product_id: i32,
    quantity: i32,
) -> Result<(), BoxError> {
    let product = products::table
        .find(product_id)
        .first::<Product>(conn)
        .map_err(|e| Box::new(e) as BoxError)?;

    let total_amount = product.price * BigDecimal::from(quantity);
    let new_order = NewOrder::new(
        user_id,
        "pending".to_string(),
        total_amount,
    );

    diesel::insert_into(orders::table)
        .values(&new_order)
        .execute(conn)
        .map(|_| ())
        .map_err(|e| Box::new(e) as BoxError)
}

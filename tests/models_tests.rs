use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;
use std::time::Instant;
use log::{info, error};
use rust_market::test_helpers::setup;
use rust_market::models::*;
use rust_market::schema::*;
use rust_market::logging::{log_performance_metrics, PerformanceMetric, MetricType};
use diesel::Connection;

// Database Setup Functions
fn setup_test_db() -> PgConnection {
    info!("Starting setup_test_db");
    
    if dotenv::from_filename(".env.test").is_err() {
        info!("Failed to load .env.test, falling back to .env");
        dotenv().ok();
    }

    let database_url = env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set in .env.test file");
    
    info!("Attempting to connect to test database: {}", 
          database_url.replace(|c: char| c != '@' && c != ':' && !c.is_ascii_alphabetic(), "*"));
    
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to database: {}", e));

    info!("Successfully established database connection");

    match conn.begin_test_transaction() {
        Ok(_) => {
            info!("Successfully started test transaction");
            conn
        },
        Err(e) => {
            error!("Failed to begin test transaction: {}", e);
            panic!("Failed to begin test transaction: {}", e);
        }
    }
}

// Helper Functions
fn generate_unique_identifier() -> String {
    Uuid::new_v4().to_string().split('-').next().unwrap().to_string()
}

fn insert_bulk_users(connection: &mut PgConnection, num_users: usize) {
    let unique_id = generate_unique_identifier();
    let new_users: Vec<NewUser> = (0..num_users).map(|i| {
        NewUser::new(
            format!("Bulk User {} {}", unique_id, i),
            format!("bulkuser{}{}@example.com", unique_id, i),
            "hashed_password".to_string(),
        )
    }).collect();

    diesel::insert_into(users::table)
        .values(&new_users)
        .execute(connection)
        .expect("Failed to insert users");
}

fn insert_bulk_products(connection: &mut PgConnection, num_products: usize) {
    let unique_id = generate_unique_identifier();
    let new_products: Vec<NewProduct> = (0..num_products).map(|i| {
        NewProduct::new(
            format!("Bulk Product {} {}", unique_id, i),
            Some(format!("Description for product {} {}", unique_id, i)),
            BigDecimal::from_str("9.99").expect("Invalid price"),
            100,
        )
    }).collect();

    diesel::insert_into(products::table)
        .values(&new_products)
        .execute(connection)
        .expect("Failed to insert products");
}

fn insert_bulk_orders_with_items(connection: &mut PgConnection, num_orders: usize) {
    let user_ids: Vec<i32> = users::table
        .select(users::id)
        .limit(num_orders as i64)
        .load(connection)
        .expect("Failed to retrieve user IDs");

    let product_ids: Vec<i32> = products::table
        .select(products::id)
        .load(connection)
        .expect("Failed to retrieve product IDs");

    if product_ids.is_empty() {
        panic!("No products found to create orders");
    }

    // Add type annotations for vectors
    let mut new_orders: Vec<NewOrder> = Vec::with_capacity(num_orders);
    let mut new_order_items: Vec<NewOrderItem> = Vec::new();

    for (i, user_id) in user_ids.into_iter().enumerate() {
        let order = NewOrder::new(
            user_id,
            "Pending".to_string(),
            BigDecimal::from_str("0.00").unwrap(),
        );

        let inserted_order: Order = diesel::insert_into(orders::table)
            .values(&order)
            .get_result(connection)
            .expect("Failed to insert order");

        // Add 1-3 items per order
        for _ in 0..(i % 3 + 1) {
            let product_id = product_ids[i % product_ids.len()];
            let quantity = (i % 5 + 1) as i32;

            let new_item = NewOrderItem {
                order_id: inserted_order.id,
                product_id,
                quantity,
                price_at_time: BigDecimal::from_str("9.99").unwrap(),
            };

            new_order_items.push(new_item);
        }
    }

    // Bulk insert order items
    diesel::insert_into(order_items::table)
        .values(&new_order_items)
        .execute(connection)
        .expect("Failed to insert order items");
}

// Query struct for better organization
#[derive(Debug)]
struct UserQuery;

impl UserQuery {
    fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
    }

    fn find_by_username(conn: &mut PgConnection, username: &str) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .first(conn)
    }
}

// Tests Section
#[test]
fn test_create_new_user() {
    info!("Starting test_create_new_user");
    setup();
    info!("Setup completed");
    
    let mut connection = setup_test_db();
    info!("Database connection established");
    
    let unique_id = generate_unique_identifier();
    let start = Instant::now();

    info!("Starting test_create_new_user");

    let new_user = NewUser::new(
        format!("Test User {}", unique_id),
        format!("testuser{}@example.com", unique_id),
        "hashed_password".to_string(),
    );

    let result = UserQuery::create(&mut connection, &new_user);
    let duration = start.elapsed();
    
    match result {
        Ok(user) => {
            log_performance_metrics(PerformanceMetric::new(
                "create_user",
                duration,
                true,
                MetricType::Database,
                Some(format!("User ID: {}", user.id))
            ));
            
            assert_eq!(user.username, new_user.username);
            assert_eq!(user.email, new_user.email);
            assert_eq!(user.password_hash, new_user.password_hash);
        }
        Err(e) => {
            log_performance_metrics(PerformanceMetric::new(
                "create_user",
                duration,
                false,
                MetricType::Database,
                Some(format!("Error: {:?}", e))
            ));
            panic!("Failed to insert new user: {:?}", e);
        }
    }

    info!("Completed test_create_new_user");
}

#[test]
fn test_insert_duplicate_user() {
    setup();
    let mut connection = setup_test_db();
    let unique_id = generate_unique_identifier();

    info!("Starting test_insert_duplicate_user");

    let new_user = NewUser::new(
        format!("Duplicate User {}", unique_id),
        format!("duplicateuser{}@example.com", unique_id),
        "hashed_password".to_string(),
    );

    // First insertion should succeed
    diesel::insert_into(users::table)
        .values(&new_user.clone())  // Clone the user for second insertion
        .execute(&mut connection)
        .expect("Failed to insert new user");

    // Second insertion should fail
    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut connection);

    match result {
        Ok(_) => panic!("Duplicate user was inserted, expected failure."),
        Err(e) => assert!(e.to_string().contains("duplicate key value")),
    }

    info!("Completed test_insert_duplicate_user");
}

#[test]
fn test_create_order_with_items() {
    setup();
    let mut connection = setup_test_db();
    let unique_id = generate_unique_identifier();

    let new_user = NewUser::new(
        format!("Order User {}", unique_id),
        format!("orderuser{}@example.com", unique_id),
        "hashed_password".to_string(),
    );

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut connection)
        .expect("Failed to insert new user");

    // Create new products
    let new_product1 = NewProduct::new(
        "Product 1".to_string(),
        Some("Description 1".to_string()),
        BigDecimal::from_str("19.99").expect("Invalid price"),
        100,
    );

    let new_product2 = NewProduct::new(
        "Product 2".to_string(),
        Some("Description 2".to_string()),
        BigDecimal::from_str("29.99").expect("Invalid price"),
        50,
    );

    let product1 = diesel::insert_into(products::table)
        .values(&new_product1)
        .get_result::<Product>(&mut connection)
        .expect("Failed to insert product 1");

    let product2 = diesel::insert_into(products::table)
        .values(&new_product2)
        .get_result::<Product>(&mut connection)
        .expect("Failed to insert product 2");

    // Create a new order
    let total_amount = &product1.price + &product2.price;

    let new_order = NewOrder::new(
        user.id,
        "Pending".to_string(),
        total_amount.clone(),
    );

    let order = diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result::<Order>(&mut connection)
        .expect("Failed to create new order");

    // Create order items
    let new_order_item1 = NewOrderItem {
        order_id: order.id,
        product_id: product1.id,
        quantity: 1,
        price_at_time: product1.price.clone(),
    };

    let new_order_item2 = NewOrderItem {
        order_id: order.id,
        product_id: product2.id,
        quantity: 1,
        price_at_time: product2.price.clone(),
    };

    diesel::insert_into(order_items::table)
        .values(&new_order_item1)
        .execute(&mut connection)
        .expect("Failed to insert order item 1");

    diesel::insert_into(order_items::table)
        .values(&new_order_item2)
        .execute(&mut connection)
        .expect("Failed to insert order item 2");

    // Retrieve the order with items
    let retrieved_order = orders::table
        .find(order.id)
        .first::<Order>(&mut connection)
        .expect("Failed to retrieve order");

    assert_eq!(retrieved_order.total_amount, total_amount);

    // Load order items
    let items = OrderItem::belonging_to(&retrieved_order)
        .load::<OrderItem>(&mut connection)
        .expect("Failed to load order items");

    assert_eq!(items.len(), 2);
}

#[test]
fn test_update_product_stock() {
    setup();
    let mut connection = setup_test_db();

    // Create a new product
    let new_product = NewProduct::new(
        "Stock Product".to_string(),
        None,
        BigDecimal::from_str("9.99").expect("Invalid price"),
        20,
    );

    let product = diesel::insert_into(products::table)
        .values(&new_product)
        .get_result::<Product>(&mut connection)
        .expect("Failed to insert new product");

    // Update stock level
    let updated_stock_level = 15;

    diesel::update(products::table.filter(products::id.eq(product.id)))
        .set(products::stock_level.eq(updated_stock_level))
        .execute(&mut connection)
        .expect("Failed to update product stock level");

    // Retrieve and assert
    let updated_product = products::table
        .find(product.id)
        .first::<Product>(&mut connection)
        .expect("Failed to retrieve updated product");

    assert_eq!(updated_product.stock_level, updated_stock_level);
}

#[test]
fn test_bulk_insert_users() {
    setup();
    let mut connection = setup_test_db();

    info!("Starting test_bulk_insert_users");

    let start = Instant::now();
    let num_users = 1000; // Number of users to insert

    let mut new_users = Vec::with_capacity(num_users);

    for i in 0..num_users {
        new_users.push(NewUser::new(
            format!("Bulk User {}", i),
            format!("bulkuser{}@example.com", i),
            "hashed_password".to_string(),
        ));
    }

    let insert_start = Instant::now();
    match diesel::insert_into(users::table)
        .values(&new_users)
        .execute(&mut connection) {
        Ok(inserted) => {
            let duration = insert_start.elapsed();
            log_performance_metrics(PerformanceMetric::new(
                "bulk_insert_users",
                duration,
                true,
                MetricType::Database,
                Some(format!("Inserted {} users", inserted)),
            ));
            assert_eq!(inserted, num_users);
        },
        Err(e) => {
            let duration = insert_start.elapsed();
            log_performance_metrics(PerformanceMetric::new(
                "bulk_insert_users",
                duration,
                false,
                MetricType::Database,
                Some(format!("Error: {:?}", e)),
            ));
            panic!("Failed to insert users: {:?}", e);
        }
    }

    let total_duration = start.elapsed();
    info!("Completed test_bulk_insert_users in {:?}", total_duration);
}

#[test]
fn test_bulk_read_users() {
    setup();
    let mut connection = setup_test_db();

    info!("Starting test_bulk_read_users");

    // First, ensure we start with a clean state
    diesel::delete(users::table)
        .execute(&mut connection)
        .expect("Failed to clear users table");

    // Insert users
    info!("Inserting 1000 users...");
    insert_bulk_users(&mut connection, 1000);

    // Verify insertion
    let user_count: i64 = users::table
        .select(diesel::dsl::count_star())
        .first(&mut connection)
        .expect("Failed to count users");

    info!("User count after insertion: {}", user_count);
    assert_eq!(user_count, 1000, "Should have exactly 1000 users after insertion");

    let read_start = Instant::now();
    let users_list = users::table
        .load::<User>(&mut connection)
        .expect("Failed to load users");
    let duration = read_start.elapsed();

    let num_users = users_list.len();
    log_performance_metrics(PerformanceMetric::new(
        "bulk_read_users",
        duration,
        true,
        MetricType::Database,
        Some(format!("Read {} users", num_users)),
    ));
    
    assert_eq!(num_users, 1000, "Should have read exactly 1000 users");

    info!("Completed test_bulk_read_users");
}

#[test]
fn test_complex_query_performance() {
    setup();
    let mut connection = setup_test_db();

    info!("Starting test_complex_query_performance");

    // Insert test data using our helper functions
    insert_bulk_users(&mut connection, 1000);
    insert_bulk_products(&mut connection, 500);
    insert_bulk_orders_with_items(&mut connection, 200);

    let query_start = Instant::now();
    let results = users::table
        .inner_join(orders::table.on(orders::user_id.eq(users::id)))
        .inner_join(order_items::table.on(order_items::order_id.eq(orders::id)))
        .inner_join(products::table.on(products::id.eq(order_items::product_id)))
        .select((users::username, orders::id, products::name, order_items::quantity))
        .limit(1000)
        .load::<(String, i32, String, i32)>(&mut connection);
    let duration = query_start.elapsed();

    match results {
        Ok(records) => {
            log_performance_metrics(PerformanceMetric::new(
                "complex_query",
                duration,
                true,
                MetricType::Database,
                Some(format!("Retrieved {} records", records.len())),
            ));
            assert!(records.len() > 0);
        },
        Err(e) => {
            log_performance_metrics(PerformanceMetric::new(
                "complex_query",
                duration,
                false,
                MetricType::Database,
                Some(format!("Error: {:?}", e)),
            ));
            panic!("Failed to execute complex query: {:?}", e);
        }
    }

    info!("Completed test_complex_query_performance");
}

#[test]
fn test_bulk_update_products() {
    setup();
    let mut connection = setup_test_db();

    info!("Starting test_bulk_update_products");

    // First, ensure we start with a known state by clearing existing products
    diesel::delete(products::table)
        .execute(&mut connection)
        .expect("Failed to clear products");

    // Insert exactly 500 products
    info!("Inserting 500 products...");
    insert_bulk_products(&mut connection, 500);

    // Verify the count before update
    let product_count: i64 = products::table
        .select(diesel::dsl::count_star())
        .first(&mut connection)
        .expect("Failed to count products");

    info!("Product count before update: {}", product_count);
    assert_eq!(product_count, 500, "Should have exactly 500 products before update");

    let update_start = Instant::now();
    let result = diesel::update(products::table)
        .set(products::stock_level.eq(products::stock_level - 1))
        .execute(&mut connection);
    let duration = update_start.elapsed();

    match result {
        Ok(updated) => {
            log_performance_metrics(PerformanceMetric::new(
                "bulk_update_products",
                duration,
                true,
                MetricType::Database,
                Some(format!("Updated {} products", updated)),
            ));
            assert_eq!(updated as i64, product_count, 
                "Number of updated products should match the total count");
            
            // Verify the update was successful
            let products_below_initial: i64 = products::table
                .filter(products::stock_level.eq(99))  // Initial was 100
                .count()
                .get_result(&mut connection)
                .expect("Failed to count updated products");
            
            assert_eq!(products_below_initial, product_count, 
                "All products should have been updated");
        },
        Err(e) => {
            log_performance_metrics(PerformanceMetric::new(
                "bulk_update_products",
                duration,
                false,
                MetricType::Database,
                Some(format!("Error: {:?}", e)),
            ));
            panic!("Failed to update products: {:?}", e);
        }
    }

    info!("Completed test_bulk_update_products");
}

// Additional tests can be added here for Product, Order, and OrderItem models


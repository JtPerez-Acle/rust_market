// tests/performance_tests.rs

use diesel::prelude::*;
use rust_market::{
    models::NewUser,
    db::establish_connection_pool,
    test_helpers,
};
use std::error::Error;

#[test]
fn test_performance() {
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create connection pool");
    let conn = &mut pool.get()
        .expect("Failed to get connection");

    // Clean up before test
    test_helpers::cleanup_database(&pool);

    // Test user creation performance
    let start = std::time::Instant::now();
    let num_users = 100;
    
    for i in 0..num_users {
        let user = NewUser::new(
            format!("testuser{}", i),
            format!("test{}@example.com", i),
            "hashedpassword123".to_string(),
        );
        
        diesel::insert_into(rust_market::schema::users::table)
            .values(&user)
            .execute(conn)
            .expect("Failed to insert user");
    }
    
    let duration = start.elapsed();
    println!("Created {} users in {:?} ({:?} per user)", 
        num_users, 
        duration,
        duration / num_users
    );

    // Clean up after test
    test_helpers::cleanup_database(&pool);
}

#[test]
fn test_concurrent_user_creation() {
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create connection pool");
    
    // Clean up before test
    test_helpers::cleanup_database(&pool);
    
    let num_users = 10;
    let handles: Vec<_> = (0..num_users)
        .map(|i| {
            let pool = pool.clone();
            std::thread::spawn(move || {
                let conn = &mut pool.get().unwrap();
                let user = NewUser::new(
                    format!("concurrent_user{}", i),
                    format!("concurrent{}@example.com", i),
                    "hashedpassword123".to_string(),
                );
                
                diesel::insert_into(rust_market::schema::users::table)
                    .values(&user)
                    .execute(conn)
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    let successful = results.iter().filter(|r| r.is_ok()).count();
    println!("Successfully created {}/{} users concurrently", successful, num_users);
    
    // Clean up after test
    test_helpers::cleanup_database(&pool);
}

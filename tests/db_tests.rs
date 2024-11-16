use std::env;
use rust_market::db;
use log::{info, warn};

#[test]
fn test_establish_connection_pool_success() {
    // Load test environment
    dotenv::from_filename(".env.test").ok();
    
    let result = db::establish_connection_pool(None);
    assert!(result.is_ok(), "Connection pool should be created successfully");
}

#[test]
fn test_establish_connection_pool_missing_env() {
    // Temporarily clear both environment variables
    let original = env::var("DATABASE_URL_TEST").ok();
    env::remove_var("DATABASE_URL_TEST");
    env::remove_var("DATABASE_URL"); // Also remove this to prevent fallback

    // Test with missing environment variable
    let result = db::establish_connection_pool(None);
    
    // Restore the environment variable if it existed
    if let Some(url) = original {
        env::set_var("DATABASE_URL_TEST", url);
    }
    
    assert!(result.is_err(), "Connection pool should fail without DATABASE_URL_TEST");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("DATABASE_URL_TEST not found in environment"), 
           "Error message should indicate missing DATABASE_URL_TEST environment variable");
}

#[test]
fn test_establish_connection_pool_with_url() {
    // Use an invalid URL but in a controlled way
    let invalid_url = "postgres://fake:fake@localhost/nonexistent";
    
    info!("Testing connection pool with invalid credentials (expected to fail)");
    let result = db::establish_connection_pool(Some(invalid_url));
    
    match result {
        Err(e) => {
            warn!("Expected authentication failure: {}", e);
            assert!(e.to_string().contains("authentication failed"), 
                   "Error should be about authentication failure");
        },
        Ok(_) => panic!("Should not succeed with invalid credentials"),
    }
}

// Helper function to reset environment after tests
#[allow(dead_code)]
fn cleanup() {
    dotenv::from_filename(".env.test").ok();
}

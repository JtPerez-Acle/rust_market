use dotenv::dotenv;
use std::env;
use crate::logging;
use log::info;
use std::fs;
use chrono::Utc;
use std::sync::Once;

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

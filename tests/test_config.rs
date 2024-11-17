use std::env;
use log::warn;

// Default test configuration
const DEFAULT_DB_HOST: &str = "localhost";
const DEFAULT_DB_PORT: &str = "5432";
const DEFAULT_DB_USER: &str = "postgres";
const DEFAULT_DB_PASSWORD: &str = "postgres";
const DEFAULT_DB_NAME: &str = "rust_market_test";

pub fn generate_db_url() -> String {
    env::var("DATABASE_URL_TEST").unwrap_or_else(|_| {
        let host = env::var("TEST_DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string());
        let port = env::var("TEST_DB_PORT").unwrap_or_else(|_| DEFAULT_DB_PORT.to_string());
        let user = env::var("TEST_DB_USER").unwrap_or_else(|_| DEFAULT_DB_USER.to_string());
        let password = env::var("TEST_DB_PASSWORD").unwrap_or_else(|_| {
            warn!("Using default database password. This is not recommended for production.");
            DEFAULT_DB_PASSWORD.to_string()
        });
        let name = env::var("TEST_DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string());

        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, name
        )
    })
}

pub fn get_test_db_url() -> String {
    generate_db_url()
}

pub fn setup_test_db() {
    env::set_var("DATABASE_URL_TEST", get_test_db_url());
    
    // Set default log level if not already set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    // Enable backtrace by default for tests
    if env::var("RUST_BACKTRACE").is_err() {
        env::set_var("RUST_BACKTRACE", "1");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cleanup_env_vars() {
        env::remove_var("DATABASE_URL_TEST");
        env::remove_var("TEST_DB_HOST");
        env::remove_var("TEST_DB_PORT");
        env::remove_var("TEST_DB_USER");
        env::remove_var("TEST_DB_PASSWORD");
        env::remove_var("TEST_DB_NAME");
    }

    #[test]
    fn test_default_config() {
        cleanup_env_vars();
        
        let url = get_test_db_url();
        let expected_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            DEFAULT_DB_USER, DEFAULT_DB_PASSWORD, DEFAULT_DB_HOST, DEFAULT_DB_PORT, DEFAULT_DB_NAME
        );
        assert_eq!(url, expected_url);
        
        cleanup_env_vars();
    }

    #[test]
    fn test_custom_config() {
        cleanup_env_vars();
        
        let custom_url = "postgres://custom:pass@host:5433/testdb";
        env::set_var("DATABASE_URL_TEST", custom_url);
        let url = get_test_db_url();
        assert_eq!(url, custom_url);
        
        cleanup_env_vars();
    }

    #[test]
    fn test_setup_environment() {
        cleanup_env_vars();
        
        setup_test_db();
        assert!(env::var("RUST_LOG").is_ok());
        assert!(env::var("RUST_BACKTRACE").is_ok());
        assert!(env::var("DATABASE_URL_TEST").is_ok());
        
        let url = env::var("DATABASE_URL_TEST").unwrap();
        let expected_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            DEFAULT_DB_USER, DEFAULT_DB_PASSWORD, DEFAULT_DB_HOST, DEFAULT_DB_PORT, DEFAULT_DB_NAME
        );
        assert_eq!(url, expected_url);
        
        cleanup_env_vars();
    }
}

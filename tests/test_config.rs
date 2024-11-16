use std::env;
use once_cell::sync::Lazy;

static TEST_DB_URL: Lazy<String> = Lazy::new(|| {
    env::var("DATABASE_URL_TEST").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust_market_test".to_string()
    })
});

pub fn get_test_db_url() -> String {
    TEST_DB_URL.clone()
}

pub fn setup_test_db() {
    env::set_var("DATABASE_URL_TEST", get_test_db_url());
}

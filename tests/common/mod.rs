pub mod test_logging;

use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
use log::info;
use std::env;

pub fn setup_test_db() -> PgConnection {
    info!("Starting setup_test_db");
    dotenv::from_filename(".env.test").ok();

    let database_url = env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set in .env.test file");
    
    info!(
        "Attempting to connect to test database: {}",
        database_url.replace(
            |c: char| c != '@' && c != ':' && !c.is_ascii_alphabetic(),
            "*"
        )
    );
    
    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to database: {}", e));

    info!("Successfully established database connection");

    conn
} 
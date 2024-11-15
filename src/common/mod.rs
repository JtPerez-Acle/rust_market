pub mod test_logging;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use log::info;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn setup_test_db() -> Pool {
    info!("Starting setup_test_db");

    // Load environment variables from `.env.test`
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

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    info!("Successfully established database connection pool");

    pool
} 
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use log::info;
use std::error::Error as StdError;
use std::time::Instant;
use crate::logging::{log_performance_metrics, PerformanceMetric, MetricType};

pub mod users;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbPool = Pool;
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    ConnectionError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Error::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl StdError for Error {}

/// Establishes a connection pool to the PostgreSQL database
pub fn establish_connection_pool(database_url: Option<&str>) -> Result<DbPool, BoxError> {
    let start = Instant::now();
    
    // Get database URL
    let database_url = match database_url {
        Some(url) => url.to_string(),
        None => {
            info!("No URL provided, attempting to read from environment");
            match env::var("DATABASE_URL_TEST") {
                Ok(url) => url,
                Err(_) => {
                    log_performance_metrics(PerformanceMetric::new(
                        "establish_connection_pool",
                        start.elapsed(),
                        false,
                        MetricType::System,
                        Some("DATABASE_URL_TEST not found in environment".to_string())
                    ));
                    return Err(Box::new(Error::ConfigError(
                        "DATABASE_URL_TEST not found in environment".to_string()
                    )));
                }
            }
        }
    };

    // Create a connection manager
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

    // Create the connection pool
    match r2d2::Pool::builder().build(manager) {
        Ok(pool) => {
            log_performance_metrics(PerformanceMetric::new(
                "establish_connection_pool",
                start.elapsed(),
                true,
                MetricType::System,
                Some("Pool created successfully".to_string())
            ));
            Ok(pool)
        },
        Err(e) => {
            let safe_url = database_url.replace(
                |c: char| c != '@' && c != ':' && !c.is_ascii_alphabetic(), 
                "*"
            );
            log_performance_metrics(PerformanceMetric::new(
                "establish_connection_pool",
                start.elapsed(),
                false,
                MetricType::System,
                Some(format!("Failed to create pool for {}: {}", safe_url, e))
            ));
            Err(Box::new(Error::ConnectionError(e.to_string())))
        }
    }
}

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use log::info;
use std::error::Error as StdError;
use std::time::Instant;
use crate::logging::{log_performance_metrics, PerformanceMetric, MetricType};
use diesel::prelude::*;
use crate::models::Asset;
use crate::schema::assets;
use bigdecimal::BigDecimal;
use std::str::FromStr;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

pub fn establish_connection_pool(database_url: Option<&str>) -> Result<Pool, Box<dyn StdError>> {
    let start = Instant::now();
    
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

    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

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

pub fn get_all_assets(pool: &Pool) -> Result<Vec<Asset>, diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    assets::table.load::<Asset>(conn)
}

pub fn buy_asset(pool: &Pool, asset_id: i32, quantity: i32) -> Result<(), diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    
    conn.transaction(|conn| {
        let asset: Asset = assets::table.find(asset_id).first(conn)?;
        
        if asset.stock < quantity {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        diesel::update(assets::table.find(asset_id))
            .set(assets::stock.eq(asset.stock - quantity))
            .execute(conn)?;

        Ok(())
    })
}

pub fn sell_asset(pool: &Pool, asset_id: i32, quantity: i32) -> Result<(), diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    
    diesel::update(assets::table.find(asset_id))
        .set(assets::stock.eq(assets::stock + quantity))
        .execute(conn)?;

    Ok(())
}

pub fn update_asset_price(pool: &Pool, asset_id: i32, new_price: f64) -> Result<(), diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    
    let decimal_price = BigDecimal::from_str(&new_price.to_string())
        .map_err(|_| diesel::result::Error::RollbackTransaction)?;
    
    diesel::update(assets::table.find(asset_id))
        .set(assets::price.eq(decimal_price))
        .execute(conn)?;

    Ok(())
}

pub fn update_asset_stock(pool: &Pool, asset_id: i32, new_stock: i32) -> Result<(), diesel::result::Error> {
    let conn = &mut pool.get().unwrap();
    
    diesel::update(assets::table.find(asset_id))
        .set(assets::stock.eq(new_stock))
        .execute(conn)?;

    Ok(())
}

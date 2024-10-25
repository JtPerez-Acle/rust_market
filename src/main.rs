pub mod models;
pub mod schema;
pub mod db;
pub mod errors;
pub mod handlers;

use actix_web::{App, HttpServer, middleware::Logger as ActixLogger};
use dotenv::dotenv;
use flexi_logger::{FileSpec, Logger, WriteMode};
use handlers::health_check;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Initialize flexi_logger with correct configuration
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .suffix("log")
        )
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .expect("Failed to initialize logger");

    // Set up database connection pool
    let pool = db::establish_connection_pool()
        .expect("🛑 Failed to create database connection pool");

    log::info!("🚀 Starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(ActixLogger::default())
            .wrap(ActixLogger::new("%a %{User-Agent}i"))
            .app_data(actix_web::web::Data::new(pool.clone()))
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

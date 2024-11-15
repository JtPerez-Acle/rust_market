use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use rust_market::logging;
use rust_market::handlers;
use rust_market::db;
use rust_market::common;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    logging::init_logger().expect("Failed to initialize logger");
    info!("Rust Market Application Starting...");

    // Set up database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::health_check)
            .service(
                web::scope("/api")
                    .service(handlers::get_assets)
                    .service(handlers::buy_asset)
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

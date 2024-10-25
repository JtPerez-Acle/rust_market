use dotenv::dotenv;
use rust_market::logging;
use log::info;

fn main() {
    // Initialize environment variables
    dotenv().ok();

    // Initialize logger
    logging::init_logger().expect("Failed to initialize logger");

    // Log startup message
    info!("Rust Market Application Started");
}

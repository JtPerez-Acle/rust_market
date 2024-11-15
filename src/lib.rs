pub mod models;
pub mod schema;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod logging;
pub mod common;
pub mod migrations;
// Add other modules as needed

// Re-export items if needed
pub use common::setup_test_db;
pub use common::test_logging::{log_test_metric, TestMetric, calculate_performance_metrics};
pub use migrations::MIGRATIONS;





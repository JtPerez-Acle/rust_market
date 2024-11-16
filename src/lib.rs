pub mod models;
pub mod schema;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod logging;  // Add this line to register the logging module
pub mod test_helpers;

// Re-export test_logging for tests
#[cfg(test)]
pub mod test_logging;

// Add other modules as needed

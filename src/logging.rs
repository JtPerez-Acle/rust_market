use flexi_logger::{
    FileSpec, Logger, WriteMode, Duplicate,
};
use log::{info, debug, error};
use std::error::Error;
use std::time::Duration;
use chrono::Utc;
use std::sync::Once;
use serde_json::json;
use std::path::Path;
use chrono::Local;

// Static variable to ensure single initialization
static INIT_LOGGER: Once = Once::new();
static mut LOGGER_INITIALIZED: bool = false;

/// Initializes the logger for the application
/// Returns Result indicating success or failure of logger initialization
pub fn init_logger() -> Result<(), Box<dyn Error>> {
    unsafe {
        if LOGGER_INITIALIZED {
            return Ok(());
        }
    }

    let mut initialization_success = false;
    
    INIT_LOGGER.call_once(|| {
        let is_test = std::env::var("RUST_TEST").is_ok();
        let log_dir = if is_test {
            Path::new("tests/logs")
        } else {
            Path::new("logs")
        };

        // Create the directory if it doesn't exist
        if let Err(e) = std::fs::create_dir_all(log_dir) {
            error!("Failed to create log directory: {}", e);
            return;
        }

        let date_str = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let file_name = if is_test {
            format!("rust_market_test_{}", date_str)
        } else {
            format!("rust_market_{}", date_str)
        };

        // Initialize flexi_logger
        let logger_result = Logger::try_with_str("debug")
            .map(|logger| {
                logger
                    .log_to_file(
                        FileSpec::default()
                            .directory(log_dir)
                            .basename(file_name)
                    )
                    .write_mode(WriteMode::BufferAndFlush)
                    .start()
            });

        match logger_result {
            Ok(Ok(_)) => {
                unsafe { 
                    LOGGER_INITIALIZED = true;
                }
                initialization_success = true;
            },
            _ => {
                error!("Failed to initialize logger");
            }
        }
    });

    if initialization_success {
        Ok(())
    } else {
        Err("Logger initialization failed".into())
    }
}

// Performance metric types
#[derive(Debug)]
pub enum MetricType {
    Database,
    Api,
    Business,
    System
}

pub struct PerformanceMetric {
    pub operation: String,
    pub duration: Duration,
    pub success: bool,
    pub metric_type: MetricType,
    pub details: Option<String>,
}

impl PerformanceMetric {
    pub fn new(
        operation: impl Into<String>,
        duration: Duration,
        success: bool,
        metric_type: MetricType,
        details: Option<String>,
    ) -> Self {
        Self {
            operation: operation.into(),
            duration,
            success,
            metric_type,
            details,
        }
    }

    fn to_json(&self) -> String {
        json!({
            "timestamp": Utc::now().to_rfc3339(),
            "operation": self.operation,
            "duration_ms": self.duration.as_secs_f64() * 1000.0,
            "success": self.success,
            "type": format!("{:?}", self.metric_type),
            "details": self.details,
        }).to_string()
    }
}

pub fn log_performance_metrics(metric: PerformanceMetric) {
    let duration_formatted = format_duration(metric.duration);
    let status = if metric.success { "SUCCESS" } else { "FAILURE" };
    
    info!(
        "PERFORMANCE_METRIC - {} - Operation: {}, Status: {}, Duration: {}, Type: {:?}, Details: {}",
        Utc::now().to_rfc3339(),
        metric.operation,
        status,
        duration_formatted,
        metric.metric_type,
        metric.details.as_deref().unwrap_or("None")
    );

    // Also log as JSON for structured logging
    debug!("METRIC_JSON {}", metric.to_json());
}

/// Formats a duration for logging purposes
pub fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros() as f64;
    if micros >= 1_000_000.0 {
        format!("{:.2}s", micros / 1_000_000.0)
    } else if micros >= 1000.0 {
        format!("{:.2}ms", micros / 1000.0)
    } else {
        format!("{:.2}µs", micros)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_micros(500)), "500.00µs");
        assert_eq!(format_duration(Duration::from_millis(1500)), "1.50s");
    }

    #[test]
    fn test_logger_initialization() {
        // Set test environment variable
        std::env::set_var("RUST_TEST", "1");
        
        // First initialization
        let result1 = init_logger();
        assert!(result1.is_ok(), "First logger initialization should succeed");

        // Second initialization should succeed (but actually do nothing)
        let result2 = init_logger();
        assert!(result2.is_ok(), "Second logger initialization should succeed");
    }
}

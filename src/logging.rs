use flexi_logger::{
    FileSpec, Logger, WriteMode, Duplicate,
};
use log::{info, debug};
use std::error::Error;
use std::time::Duration;
use chrono::Utc;
use std::sync::Once;
use serde_json::json;

// Static variable to ensure single initialization
static LOGGER_INIT: Once = Once::new();

/// Initializes the logger for the application
/// Returns Result indicating success or failure of logger initialization
pub fn init_logger() -> Result<(), Box<dyn Error>> {
    LOGGER_INIT.call_once(|| {
        // Updated log specification to completely suppress r2d2 authentication errors
        let log_spec = "debug,r2d2=error,diesel=warn";  // Changed r2d2 from warn to error

        Logger::try_with_str(log_spec)
            .and_then(|logger| {
                logger
                    .log_to_file(
                        FileSpec::default()
                            // Use absolute path for the logs directory
                            .directory("/home/jtdev/Desktop/rust_projects/rust_market/logs")
                            .basename("rust_market")
                            .discriminant("r")
                            .suffix("log"),
                    )
                    .write_mode(WriteMode::BufferAndFlush)
                    .format(flexi_logger::detailed_format)
                    .duplicate_to_stderr(Duplicate::Error)
                    .start()
            })
            .unwrap_or_else(|e| {
                panic!("Failed to initialize logger: {}", e);
            });

        // Log initialization success
        info!("Logger initialized at {}", Utc::now());
        info!("Log level set to {}", log_spec);
        info!("----------------------------------------");
    });

    Ok(())
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
        // First initialization should succeed
        let result1 = init_logger();
        assert!(result1.is_ok(), "First logger initialization should succeed");

        // Second initialization should also succeed (but won't actually initialize again)
        let result2 = init_logger();
        assert!(result2.is_ok(), "Second logger initialization should succeed");
    }
}

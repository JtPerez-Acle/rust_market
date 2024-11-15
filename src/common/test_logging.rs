use std::time::Duration;
use log::{info, error};
use serde::Serialize;

#[derive(Serialize)]
pub struct TestMetric {
    pub test_name: &'static str,
    pub operation: &'static str,
    pub duration_ms: u128,
    pub success: bool,
    pub details: Option<String>,
    pub performance_data: Option<TestMetricData>,
}

impl TestMetric {
    pub fn new(
        test_name: &'static str,
        operation: &'static str,
        duration: Duration,
        success: bool,
        details: Option<String>,
        performance_data: Option<TestMetricData>,
    ) -> Self {
        Self {
            test_name,
            operation,
            duration_ms: duration.as_millis(),
            success,
            details,
            performance_data,
        }
    }
}

#[derive(Serialize)]
pub struct TestMetricData {
    pub operations_per_second: f64,
    pub average_duration_ms: f64,
    pub total_operations: usize,
    pub error_count: usize,
}

pub fn log_test_metric(metric: TestMetric) {
    if metric.success {
        info!(
            "[{}] {} - {}ms - {}",
            metric.test_name,
            metric.operation,
            metric.duration_ms,
            metric
                .details
                .as_deref()
                .unwrap_or("No additional details")
        );
    } else {
        error!(
            "[{}] {} FAILED - {}ms - {}",
            metric.test_name,
            metric.operation,
            metric.duration_ms,
            metric
                .details
                .as_deref()
                .unwrap_or("No additional details")
        );
    }
}

pub fn calculate_performance_metrics(
    operation_count: usize,
    total_duration: Duration,
    error_count: usize,
) -> TestMetricData {
    let duration_secs = total_duration.as_secs_f64();
    let operations_per_second = operation_count as f64 / duration_secs;
    let average_duration_ms = (total_duration.as_millis() as f64) / (operation_count as f64);

    TestMetricData {
        operations_per_second,
        average_duration_ms,
        total_operations: operation_count,
        error_count,
    }
} 
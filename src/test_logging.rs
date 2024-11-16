use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde_json;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
static LOG_FILE: Lazy<Mutex<Option<File>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestMetrics {
    test_name: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    duration: f64,  // in milliseconds
    success: bool,
    database_operations: usize,
    crud_stats: CrudStats,
    performance_stats: PerformanceStats,
    error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrudStats {
    creates: usize,
    reads: usize,
    updates: usize,
    deletes: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceStats {
    avg_response_time: f64,  // in milliseconds
    max_response_time: f64,
    min_response_time: f64,
    total_operations: usize,
    operations_per_second: f64,
}

impl Default for CrudStats {
    fn default() -> Self {
        Self {
            creates: 0,
            reads: 0,
            updates: 0,
            deletes: 0,
        }
    }
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            avg_response_time: 0.0,
            max_response_time: 0.0,
            min_response_time: f64::MAX,
            total_operations: 0,
            operations_per_second: 0.0,
        }
    }
}

pub struct TestLogger {
    test_name: String,
    start_time: DateTime<Utc>,
    crud_stats: CrudStats,
    performance_stats: PerformanceStats,
    response_times: Vec<f64>,
}

impl TestLogger {
    pub fn new(test_name: &str) -> Self {
        let test_number = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let test_name = format!("{}_{}", test_name, test_number);
        
        Self {
            test_name,
            start_time: Utc::now(),
            crud_stats: CrudStats::default(),
            performance_stats: PerformanceStats::default(),
            response_times: Vec::new(),
        }
    }

    pub fn log_operation(&mut self, operation_type: &str, duration: Duration) {
        let duration_ms = duration.as_secs_f64() * 1000.0;
        self.response_times.push(duration_ms);

        match operation_type {
            "create" => self.crud_stats.creates += 1,
            "read" => self.crud_stats.reads += 1,
            "update" => self.crud_stats.updates += 1,
            "delete" => self.crud_stats.deletes += 1,
            _ => {}
        }

        // Update performance stats
        self.performance_stats.total_operations += 1;
        self.performance_stats.max_response_time = self.performance_stats.max_response_time.max(duration_ms);
        self.performance_stats.min_response_time = self.performance_stats.min_response_time.min(duration_ms);
        
        let total_time: f64 = self.response_times.iter().sum();
        self.performance_stats.avg_response_time = total_time / self.response_times.len() as f64;
        
        let total_duration = (Utc::now() - self.start_time).num_seconds() as f64;
        if total_duration > 0.0 {
            self.performance_stats.operations_per_second = self.performance_stats.total_operations as f64 / total_duration;
        }
    }

    pub fn finish(&self, success: bool, error_message: Option<String>) {
        let end_time = Utc::now();
        let duration = (end_time - self.start_time).num_milliseconds() as f64;

        let metrics = TestMetrics {
            test_name: self.test_name.clone(),
            start_time: self.start_time,
            end_time,
            duration,
            success,
            database_operations: self.performance_stats.total_operations,
            crud_stats: self.crud_stats.clone(),
            performance_stats: self.performance_stats.clone(),
            error_message,
        };

        self.save_metrics(&metrics);
    }

    fn save_metrics(&self, metrics: &TestMetrics) {
        let log_dir = Path::new("tests/logs");
        if !log_dir.exists() {
            fs::create_dir_all(log_dir).expect("Failed to create logs directory");
        }

        let mut log_file = LOG_FILE.lock().unwrap();
        if log_file.is_none() {
            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            let file_path = log_dir.join(format!("test_metrics_{}.jsonl", timestamp));
            *log_file = Some(File::create(&file_path).expect("Failed to create log file"));
        }

        if let Some(file) = log_file.as_mut() {
            let json = serde_json::to_string(&metrics).expect("Failed to serialize metrics");
            writeln!(file, "{}", json).expect("Failed to write metrics to log file");
            file.flush().expect("Failed to flush log file");
        }
    }
}

#[macro_export]
macro_rules! log_test {
    ($test_name:expr, $test_code:expr) => {{
        let mut logger = TestLogger::new($test_name);
        let start = Instant::now();
        let result = std::panic::catch_unwind(|| $test_code);
        let duration = start.elapsed();

        match result {
            Ok(_) => {
                logger.log_operation($test_name, duration);
                logger.finish(true, None);
            }
            Err(e) => {
                let error_msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown error".to_string()
                };
                logger.finish(false, Some(error_msg));
                std::panic::resume_unwind(e);
            }
        }
    }};
}

// Helper function to measure performance degradation
pub fn measure_insertion_limit<F>(mut operation: F) -> (usize, Duration)
where
    F: FnMut() -> Result<(), Box<dyn std::error::Error>>,
{
    let threshold_ms = 1000.0; // 1 second threshold
    let mut count = 0;
    let start = Instant::now();

    loop {
        let op_start = Instant::now();
        if let Err(_) = operation() {
            break;
        }
        let duration = op_start.elapsed();
        count += 1;

        if duration.as_secs_f64() * 1000.0 > threshold_ms {
            break;
        }
    }

    (count, start.elapsed())
}

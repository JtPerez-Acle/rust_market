use std::fs::{self, File};
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStats {
    pub operation_type: String,
    pub count: u32,
    pub total_duration_ms: u128,
    pub min_duration_ms: u128,
    pub max_duration_ms: u128,
    pub avg_duration_ms: f64,
    pub error_count: u32,
    pub latency_distribution: HashMap<String, u32>, // Buckets: "0-10ms", "10-50ms", "50-100ms", "100-500ms", ">500ms"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyMetrics {
    pub peak_concurrent_operations: u32,
    pub avg_concurrent_operations: f64,
    pub total_timeouts: u32,
    pub total_conflicts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub resource_type: String,  // "product", "user", etc.
    pub total_reads: u32,
    pub total_writes: u32,
    pub read_write_ratio: f64,
    pub contention_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub test_name: String,
    pub test_description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub success: bool,
    pub operation_stats: HashMap<String, OperationStats>,
    pub error_message: Option<String>,
    pub total_operations: u32,
    pub operations_per_second: f64,
    pub concurrent_users: Option<u32>,
    pub concurrency_metrics: Option<ConcurrencyMetrics>,
    pub resource_metrics: Vec<ResourceMetrics>,
}

impl OperationStats {
    fn new(operation_type: String) -> Self {
        let mut latency_distribution = HashMap::new();
        latency_distribution.insert("0-10ms".to_string(), 0);
        latency_distribution.insert("10-50ms".to_string(), 0);
        latency_distribution.insert("50-100ms".to_string(), 0);
        latency_distribution.insert("100-500ms".to_string(), 0);
        latency_distribution.insert(">500ms".to_string(), 0);

        OperationStats {
            operation_type,
            count: 0,
            total_duration_ms: 0,
            min_duration_ms: u128::MAX,
            max_duration_ms: 0,
            avg_duration_ms: 0.0,
            error_count: 0,
            latency_distribution,
        }
    }

    fn update_latency_distribution(&mut self, duration_ms: u128) {
        let bucket = match duration_ms {
            0..=10 => "0-10ms",
            11..=50 => "10-50ms",
            51..=100 => "50-100ms",
            101..=500 => "100-500ms",
            _ => ">500ms",
        };
        *self.latency_distribution.get_mut(bucket).unwrap() += 1;
    }
}

#[derive(Debug, Clone)]
pub struct TestLogger {
    stats: PerformanceStats,
    test_id: usize,
    operation_durations: HashMap<String, Vec<u128>>,
    error_counts: HashMap<String, u32>,
    shared: Option<Arc<Mutex<TestLogger>>>,
    current_concurrent_ops: u32,
    peak_concurrent_ops: u32,
    total_timeouts: u32,
    total_conflicts: u32,
    operation_timestamps: Vec<(u64, u32)>, // (timestamp, concurrent_ops)
}

impl TestLogger {
    pub fn new(test_name: &str, description: &str) -> Self {
        let test_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        TestLogger {
            stats: PerformanceStats {
                test_name: test_name.to_string(),
                test_description: description.to_string(),
                start_time,
                end_time: 0,
                success: false,
                operation_stats: HashMap::new(),
                error_message: None,
                total_operations: 0,
                operations_per_second: 0.0,
                concurrent_users: None,
                concurrency_metrics: None,
                resource_metrics: Vec::new(),
            },
            test_id,
            operation_durations: HashMap::new(),
            error_counts: HashMap::new(),
            shared: None,
            current_concurrent_ops: 0,
            peak_concurrent_ops: 0,
            total_timeouts: 0,
            total_conflicts: 0,
            operation_timestamps: Vec::new(),
        }
    }

    pub fn new_shared(test_name: &str, description: &str) -> Arc<Mutex<Self>> {
        let logger = TestLogger::new(test_name, description);
        let shared = Arc::new(Mutex::new(logger));
        {
            let mut guard = shared.lock().unwrap();
            guard.shared = Some(shared.clone());
        }
        shared
    }

    pub fn start_operation(&mut self) {
        self.current_concurrent_ops += 1;
        self.peak_concurrent_ops = self.peak_concurrent_ops.max(self.current_concurrent_ops);
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.operation_timestamps.push((timestamp, self.current_concurrent_ops));
    }

    pub fn end_operation(&mut self) {
        self.current_concurrent_ops = self.current_concurrent_ops.saturating_sub(1);
    }

    pub fn log_operation(&mut self, operation: &str, duration: Duration) {
        self.start_operation();
        let duration_ms = duration.as_millis();
        
        let stats = self.stats.operation_stats
            .entry(operation.to_string())
            .or_insert_with(|| OperationStats::new(operation.to_string()));
        
        stats.count += 1;
        stats.total_duration_ms += duration_ms;
        stats.min_duration_ms = stats.min_duration_ms.min(duration_ms);
        stats.max_duration_ms = stats.max_duration_ms.max(duration_ms);
        stats.avg_duration_ms = stats.total_duration_ms as f64 / stats.count as f64;
        stats.update_latency_distribution(duration_ms);

        self.operation_durations
            .entry(operation.to_string())
            .or_default()
            .push(duration_ms);
        self.stats.total_operations += 1;

        self.end_operation();

        if let Some(shared) = &self.shared {
            if let Ok(mut parent) = shared.lock() {
                parent.log_operation(operation, duration);
            }
        }
    }

    pub fn log_timeout(&mut self) {
        self.total_timeouts += 1;
    }

    pub fn log_conflict(&mut self) {
        self.total_conflicts += 1;
    }

    pub fn log_resource_metrics(&mut self, resource_type: &str, reads: u32, writes: u32, contention_points: Vec<String>) {
        let read_write_ratio = if writes > 0 {
            reads as f64 / writes as f64
        } else {
            f64::INFINITY
        };

        let metrics = ResourceMetrics {
            resource_type: resource_type.to_string(),
            total_reads: reads,
            total_writes: writes,
            read_write_ratio,
            contention_points,
        };

        self.stats.resource_metrics.push(metrics);
    }

    pub fn log_error(&mut self, operation: &str, error_msg: &str) {
        *self.error_counts.entry(operation.to_string()).or_insert(0) += 1;
        println!("Error in {}: {}", operation, error_msg);
    }

    pub fn set_concurrent_users(&mut self, count: u32) {
        self.stats.concurrent_users = Some(count);
    }

    pub fn finish(&mut self, success: bool, error_msg: Option<String>) {
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.stats.end_time = end_time;
        self.stats.success = success;
        self.stats.error_message = error_msg;

        // Calculate average concurrent operations
        let avg_concurrent_ops = if !self.operation_timestamps.is_empty() {
            let total: u32 = self.operation_timestamps.iter().map(|(_, ops)| ops).sum();
            total as f64 / self.operation_timestamps.len() as f64
        } else {
            0.0
        };

        self.stats.concurrency_metrics = Some(ConcurrencyMetrics {
            peak_concurrent_operations: self.peak_concurrent_ops,
            avg_concurrent_operations: avg_concurrent_ops,
            total_timeouts: self.total_timeouts,
            total_conflicts: self.total_conflicts,
        });

        // Calculate operations per second
        let test_duration = (end_time - self.stats.start_time) as f64;
        if test_duration > 0.0 {
            self.stats.operations_per_second = self.stats.total_operations as f64 / test_duration;
        }

        // Only write to file if this is not a child logger
        if self.shared.is_none() {
            let logs_dir = PathBuf::from("tests/logs");
            fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");

            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let file_path = logs_dir.join(format!("{}_test_metrics_{}.json", timestamp, self.test_id));

            let json = serde_json::to_string_pretty(&self.stats)
                .expect("Failed to serialize performance stats");

            let mut file = File::create(file_path).expect("Failed to create log file");
            writeln!(file, "{}", json).expect("Failed to write to log file");
        }
    }
}

pub fn measure_insertion_limit<F>(mut operation: F) -> (u32, Duration)
where
    F: FnMut() -> Result<(), Box<dyn std::error::Error>>,
{
    let start = std::time::Instant::now();
    let mut count = 0;
    let duration = Duration::from_secs(1); // Run for 1 second

    while start.elapsed() < duration {
        if let Ok(()) = operation() {
            count += 1;
        }
    }

    (count, start.elapsed())
}

#!/bin/bash

# Path to logs directory
LOGS_DIR="/home/jtdev/Desktop/rust_projects/rust_market/logs"

# Function to get the latest log file
get_latest_log_file() {
    # Find the latest log file based on modification time
    latest_log=$(ls -t "$LOGS_DIR"/rust_market_*.log 2>/dev/null | head -n1)
}

# Function to display available log files
show_log_files() {
    echo "Available log files:"
    ls -lh "$LOGS_DIR"
}

# Function to display the latest log file
show_latest_log() {
    get_latest_log_file
    if [ -f "$latest_log" ]; then
        echo "=== Latest Log File: $(basename "$latest_log") ==="
        cat "$latest_log"
    else
        echo "No log files found"
    fi
}

# Function to display performance metrics
show_performance_metrics() {
    get_latest_log_file
    if [ -f "$latest_log" ]; then
        echo "=== Performance Metrics from $(basename "$latest_log") ==="
        echo "Standard Metrics:"
        grep "PERFORMANCE_METRIC" "$latest_log" | \
        awk '{
            split($0, a, "Operation: ");
            split(a[2], b, ", Status:");
            split(b[2], c, ", Duration:");
            split(c[2], d, ", Type:");
            split(d[2], e, ", Details:");
            printf "Operation: %-20s Status: %-10s Duration: %-15s Type: %-10s Details: %s\n", 
                   b[1], c[1], d[1], e[1], e[2];
        }'
        
        echo -e "\nJSON Metrics:"
        grep "METRIC_JSON" "$latest_log" | \
        awk '{
            $1=""; $2="";  # Remove log level and METRIC_JSON prefix
            print | "jq ."  # Pretty print JSON
        }'
    else
        echo "No log files found"
    fi
}

# Main menu
while true; do
    echo
    echo "Log Viewer Menu:"
    echo "1. List all log files"
    echo "2. View latest log"
    echo "3. Show performance metrics"
    echo "4. Exit"
    read -p "Choose an option (1-4): " choice

    case $choice in
        1) show_log_files ;;
        2) show_latest_log ;;
        3) show_performance_metrics ;;
        4) exit 0 ;;
        *) echo "Invalid option" ;;
    esac
done

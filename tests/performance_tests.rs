// tests/performance_tests.rs

use std::time::{Duration, Instant};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use log::{error, info};
use std::env;
use chrono::Utc;

// Import models and schema
use rust_market::models::{NewUser, User};
use rust_market::schema::users;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Establish database connection
    let mut connection = PgConnection::establish(&database_url)
        .expect("Error connecting to the database");

    // Run performance tests
    run_performance_tests(&mut connection)?;

    Ok(())
}

fn run_performance_tests(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    // Vectors to store test results
    let mut write_times = Vec::new();
    let mut read_times = Vec::new();

    // Number of iterations for testing
    let iterations = 1000;

    // **Write Performance Test**
    for i in 0..iterations {
        let new_user = NewUser::new(
            format!("User {}", i),
            format!("user{}@example.com", i),
            format!("hash{}", i),
        );

        let start_time = Instant::now();

        let result = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);

        match result {
            Ok(_) => (),
            Err(e) => error!("Failed to insert new user: {:?}", e),
        }

        let duration = start_time.elapsed();
        write_times.push(duration);
    }

    // **Read Performance Test**
    for _ in 0..iterations {
        let start_time = Instant::now();

        let result: Result<Vec<User>, diesel::result::Error> = users::table
            .limit(100)
            .load(conn);

        match result {
            Ok(_) => (),
            Err(e) => error!("Failed to load users: {:?}", e),
        }

        let duration = start_time.elapsed();
        read_times.push(duration);
    }

    // **Calculate and Log Results**
    let total_write_time = write_times.iter().sum::<Duration>();
    let total_read_time = read_times.iter().sum::<Duration>();

    let average_write_time = total_write_time / iterations as u32;
    let average_read_time = total_read_time / iterations as u32;

    info!("Performance Test Results:");
    info!("-------------------------");
    info!("Total iterations: {}", iterations);
    info!("Average write time: {:?}", average_write_time);
    info!("Average read time: {:?}", average_read_time);
    info!("Total write time: {:?}", total_write_time);
    info!("Total read time: {:?}", total_read_time);

    Ok(())
}

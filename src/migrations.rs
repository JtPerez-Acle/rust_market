use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::PgConnection;
use std::error::Error;
use std::fmt;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// Define our own error type that can wrap both error types
#[derive(Debug)]
pub struct MigrationCustomError(String);

impl fmt::Display for MigrationCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Migration error: {}", self.0)
    }
}

impl Error for MigrationCustomError {}

pub fn run_migrations(connection: &mut PgConnection) -> Result<(), MigrationCustomError> {
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|e| MigrationCustomError(e.to_string()))?;
    Ok(())
} 
use thiserror::Error;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use log::error;
use serde_json::json;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] diesel::r2d2::Error),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Conflict: {0}")]
    Conflict(String),
}

impl From<diesel::result::Error> for ServiceError {
    fn from(error: diesel::result::Error) -> ServiceError {
        match error {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info
            ) => ServiceError::Conflict(info.message().to_string()),
            _ => ServiceError::DatabaseError(error.to_string()),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::DatabaseError(msg) => {
                error!("Database error: {}", msg);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(json!({
                    "error": "Database Error",
                    "message": msg
                }))
            }
            ServiceError::ConnectionError(err) => {
                error!("Database connection error: {}", err);
                HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(json!({
                    "error": "Service Unavailable",
                    "message": "Database connection error"
                }))
            }
            ServiceError::ValidationError(msg) => {
                error!("Validation error: {}", msg);
                HttpResponse::build(StatusCode::BAD_REQUEST).json(json!({
                    "error": "Validation Error",
                    "message": msg
                }))
            }
            ServiceError::NotFound(msg) => {
                error!("Not found: {}", msg);
                HttpResponse::build(StatusCode::NOT_FOUND).json(json!({
                    "error": "Not Found",
                    "message": msg
                }))
            }
            ServiceError::Unauthorized(msg) => {
                error!("Unauthorized: {}", msg);
                HttpResponse::build(StatusCode::UNAUTHORIZED).json(json!({
                    "error": "Unauthorized",
                    "message": msg
                }))
            }
            ServiceError::BadRequest(msg) => {
                error!("Bad request: {}", msg);
                HttpResponse::build(StatusCode::BAD_REQUEST).json(json!({
                    "error": "Bad Request",
                    "message": msg
                }))
            }
            ServiceError::InternalServerError(msg) => {
                error!("Internal server error: {}", msg);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(json!({
                    "error": "Internal Server Error",
                    "message": msg
                }))
            }
            ServiceError::Conflict(msg) => {
                error!("Conflict: {}", msg);
                HttpResponse::build(StatusCode::CONFLICT).json(json!({
                    "error": "Conflict",
                    "message": msg
                }))
            }
        }
    }
}

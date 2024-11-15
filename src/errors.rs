use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use log::error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Database error: {0}")]
    DieselError(#[from] diesel::result::Error),
    #[error("Actix error: {0}")]
    ActixError(#[from] actix_web::Error),
    #[error("Database connection error: {0}")]
    DatabaseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::DatabaseError(_) => {
                HttpResponse::ServiceUnavailable().body("Database connection failed")
            }
            MyError::ValidationError(_) => {
                HttpResponse::BadRequest().body("Invalid input provided")
            }
            MyError::DieselError(_) => {
                HttpResponse::InternalServerError().body("A database error occurred")
            }
            MyError::ActixError(_) => {
                HttpResponse::InternalServerError().body("An internal server error occurred")
            }
        }
    }
}

#[derive(Debug, derive_more::Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                error!("Internal server error");
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::NotFound(msg) => {
                error!("Not found: {}", msg);
                HttpResponse::NotFound().json(msg)
            }
            AppError::BadRequest(msg) => {
                error!("Bad request: {}", msg);
                HttpResponse::BadRequest().json(msg)
            }
        }
    }
}

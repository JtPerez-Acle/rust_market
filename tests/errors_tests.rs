use actix_web::{HttpResponse, ResponseError};
use rust_market::errors::MyError;

#[test]
fn test_my_error_response_database_error() {
    // Create a MyError instance representing a database error
    let db_error = MyError::DatabaseError("Connection failed".into());

    // Get the HTTP response from the error
    let response = db_error.error_response();

    // Assert that the response status is 503 Service Unavailable
    assert_eq!(
        response.status(),
        HttpResponse::ServiceUnavailable().finish().status(),
        "Expected HTTP 503 Service Unavailable"
    );

    // Additional assertions can be made on the response body if necessary
}

#[test]
fn test_my_error_response_validation_error() {
    // Create a MyError instance representing a validation error
    let validation_error = MyError::ValidationError("Invalid input".into());

    // Get the HTTP response from the error
    let response = validation_error.error_response();

    // Assert that the response status is 400 Bad Request
    assert_eq!(
        response.status(),
        HttpResponse::BadRequest().finish().status(),
        "Expected HTTP 400 Bad Request"
    );

    // Additional assertions can be made on the response body if necessary
}

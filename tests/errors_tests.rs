use actix_web::ResponseError;
use rust_market::errors::ServiceError;

#[test]
fn test_service_error_response_database_error() {
    let db_error = ServiceError::DatabaseError("Connection failed".into());
    let response = db_error.error_response();
    
    assert_eq!(response.status().as_u16(), 500);
}

#[test]
fn test_service_error_response_validation_error() {
    let validation_error = ServiceError::ValidationError("Username must be between 1 and 50 characters".into());
    let response = validation_error.error_response();
    
    assert_eq!(response.status().as_u16(), 400);
}

#[test]
fn test_service_error_response_connection_error() {
    let connection_error = ServiceError::DatabaseError("Failed to connect to database".into());
    let response = connection_error.error_response();
    
    assert_eq!(response.status().as_u16(), 500);
}

#[test]
fn test_service_error_response_not_found() {
    let not_found_error = ServiceError::NotFound("User not found".into());
    let response = not_found_error.error_response();
    
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn test_service_error_response_unauthorized() {
    let unauthorized_error = ServiceError::Unauthorized("Invalid token".into());
    let response = unauthorized_error.error_response();
    
    assert_eq!(response.status().as_u16(), 401);
}

#[test]
fn test_service_error_response_bad_request() {
    let bad_request_error = ServiceError::BadRequest("Invalid input format".into());
    let response = bad_request_error.error_response();
    
    assert_eq!(response.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_create_user_validation_error() {
    use actix_web::{test, App, web};
    use rust_market::handlers::create_user;
    use rust_market::models::NewUser;
    use rust_market::db::establish_connection_pool;
    use rust_market::test_helpers;
    
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&NewUser::new(
            "".to_string(),
            "test@example.com".to_string(),
            "hashedpassword123".to_string(),
        ))
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Validation Error");
    assert!(body["message"].as_str().unwrap().contains("Username must be between 1 and 50 characters"));
}

#[actix_web::test]
async fn test_create_user_invalid_email() {
    use actix_web::{test, App, web};
    use rust_market::handlers::create_user;
    use rust_market::models::NewUser;
    use rust_market::db::establish_connection_pool;
    use rust_market::test_helpers;
    
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&NewUser::new(
            "testuser".to_string(),
            "invalid-email".to_string(),
            "hashedpassword123".to_string(),
        ))
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Validation Error");
    assert!(body["message"].as_str().unwrap().contains("Invalid email format"));
}

#[actix_web::test]
async fn test_create_user_invalid_password() {
    use actix_web::{test, App, web};
    use rust_market::handlers::create_user;
    use rust_market::models::NewUser;
    use rust_market::db::establish_connection_pool;
    use rust_market::test_helpers;
    
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&NewUser::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "short".to_string(),
        ))
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Validation Error");
    assert!(body["message"].as_str().unwrap().contains("Password hash must be at least 8 characters"));
}

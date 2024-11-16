use actix_web::{test, http::StatusCode, App, web};
use rust_market::{
    handlers::{health_check, create_user},
    models::NewUser,
    db::establish_connection_pool,
    test_helpers,
    schema::users,
};
use diesel::prelude::*;
use log::info;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new()
            .service(web::resource("/health").route(web::get().to(health_check)))
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body, serde_json::json!({ "status": "ok" }));
}

#[actix_web::test]
async fn test_health_check_wrong_method() {
    let app = test::init_service(
        App::new()
            .service(web::resource("/health").route(web::get().to(health_check)))
    ).await;

    let req = test::TestRequest::post().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(
        resp.status(), 
        StatusCode::METHOD_NOT_ALLOWED,
        "Expected 405 Method Not Allowed for POST request to GET-only endpoint"
    );
}

#[actix_web::test]
async fn test_create_user() {
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    // Get a connection and start a transaction
    let conn = &mut pool.get().expect("Failed to get db connection");
    let _transaction = conn.begin_test_transaction().expect("Failed to start test transaction");
    
    // Clean up the database before the test
    info!("Starting test_create_user - cleaning database");
    test_helpers::cleanup_database(&pool);
    
    // Verify the cleanup by checking if any users exist
    let existing_users: i64 = users::table.count().get_result(conn).expect("Failed to count users");
    info!("Users in database after cleanup: {}", existing_users);
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    // First attempt should succeed
    let user_data = NewUser::new(
        "testuser".to_string(),
        "test@example.com".to_string(),
        "hashedpassword123".to_string(),
    );

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "First user creation should succeed");
    
    let user: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(user["username"], "testuser");
    assert_eq!(user["email"], "test@example.com");

    // Second attempt with same username should fail with 409 Conflict
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT, "Second user creation should fail with conflict");
    
    // The transaction will be automatically rolled back when it goes out of scope
}

#[actix_web::test]
async fn test_create_user_invalid_username() {
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    // Get a connection and start a transaction
    let conn = &mut pool.get().expect("Failed to get db connection");
    let _transaction = conn.begin_test_transaction().expect("Failed to start test transaction");
    
    // Clean up the database before the test
    info!("Starting test_create_user_invalid_username - cleaning database");
    test_helpers::cleanup_database(&pool);
    
    // Verify the cleanup by checking if any users exist
    let existing_users: i64 = users::table.count().get_result(conn).expect("Failed to count users");
    info!("Users in database after cleanup: {}", existing_users);
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    let user_data = NewUser::new(
        "".to_string(), // Empty username
        "test@example.com".to_string(),
        "hashedpassword123".to_string(),
    );

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Validation Error");
    
    // The transaction will be automatically rolled back when it goes out of scope
}

#[actix_web::test]
async fn test_create_user_invalid_email() {
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    // Get a connection and start a transaction
    let conn = &mut pool.get().expect("Failed to get db connection");
    let _transaction = conn.begin_test_transaction().expect("Failed to start test transaction");
    
    // Clean up the database before the test
    info!("Starting test_create_user_invalid_email - cleaning database");
    test_helpers::cleanup_database(&pool);
    
    // Verify the cleanup by checking if any users exist
    let existing_users: i64 = users::table.count().get_result(conn).expect("Failed to count users");
    info!("Users in database after cleanup: {}", existing_users);
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    let user_data = NewUser::new(
        "testuser".to_string(),
        "invalid-email".to_string(),
        "hashedpassword123".to_string(),
    );

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Validation Error");
    
    // The transaction will be automatically rolled back when it goes out of scope
}

#[actix_web::test]
async fn test_create_user_invalid_password() {
    test_helpers::setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create test pool");
    
    // Get a connection and start a transaction
    let conn = &mut pool.get().expect("Failed to get db connection");
    let _transaction = conn.begin_test_transaction().expect("Failed to start test transaction");
    
    // Clean up the database before the test
    info!("Starting test_create_user_invalid_password - cleaning database");
    test_helpers::cleanup_database(&pool);
    
    // Verify the cleanup by checking if any users exist
    let existing_users: i64 = users::table.count().get_result(conn).expect("Failed to count users");
    info!("Users in database after cleanup: {}", existing_users);
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(create_user)))
    ).await;

    let user_data = NewUser::new(
        "testuser".to_string(),
        "test@example.com".to_string(),
        "short".to_string(),
    );

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["error"], "Validation Error");
    
    // The transaction will be automatically rolled back when it goes out of scope
}

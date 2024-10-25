use actix_web::{test, http::StatusCode, App};
use rust_market::handlers::health_check;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    assert_eq!(
        body,
        r#"{"status":"healthy"}"#.as_bytes(),
        "Unexpected response body"
    );
}

#[actix_web::test]
async fn test_health_check_wrong_method() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    // Test with a different HTTP method on the same path
    let req = test::TestRequest::post().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    // We should get NOT_FOUND (404) because the POST method isn't registered
    assert_eq!(
        resp.status(), 
        StatusCode::NOT_FOUND,
        "Expected 404 Not Found for POST request to GET-only endpoint"
    );
}

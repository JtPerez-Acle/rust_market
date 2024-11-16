use actix_web::{web, HttpResponse, Responder};
use crate::models::NewUser;
use crate::db;
use crate::errors::ServiceError;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub async fn create_user(
    pool: web::Data<db::DbPool>,
    user_data: web::Json<NewUser>,
) -> Result<impl Responder, ServiceError> {
    // Validate username
    if user_data.username.is_empty() || user_data.username.len() > 50 {
        return Err(ServiceError::ValidationError(
            "Username must be between 1 and 50 characters".into()
        ));
    }

    // Validate email
    if !user_data.email.contains('@') || user_data.email.len() > 100 {
        return Err(ServiceError::ValidationError(
            "Invalid email format or length".into()
        ));
    }

    // Validate password hash
    if user_data.password_hash.len() < 8 {
        return Err(ServiceError::ValidationError(
            "Password hash must be at least 8 characters".into()
        ));
    }

    let user = db::users::create_user(&pool, user_data.into_inner())?;
    Ok(HttpResponse::Created().json(user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, web};
    use actix_web::http::StatusCode;
    use crate::test_helpers;
    use crate::models::User;
    use crate::db::establish_connection_pool;

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/health").route(web::get().to(health_check)))
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body, serde_json::json!({ "status": "ok" }));
    }

    #[actix_web::test]
    async fn test_create_user() {
        test_helpers::setup();
        let pool = establish_connection_pool(None)
            .expect("Failed to create test db pool");
        
        // Clean up database before test
        test_helpers::cleanup_database(&pool);
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(web::resource("/users").route(web::post().to(create_user)))
        ).await;
        
        let user_data = NewUser::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashedpassword123".to_string(),
        );
        
        // First creation should succeed
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&user_data)
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        
        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        
        // Second creation with same username should fail with conflict
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&user_data)
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CONFLICT);
    }

}

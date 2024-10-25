use actix_web::get;

#[get("/health")]
pub async fn health_check() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

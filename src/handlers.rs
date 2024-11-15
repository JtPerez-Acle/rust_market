use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::db::Pool;
use crate::models::{Asset, NewAsset};
use crate::schema::assets::dsl::*;
use crate::errors::MyError;
use log::{error, info};
use serde_json::json;
use chrono::{Utc, NaiveDateTime};

#[get("/api/assets")]
pub async fn get_assets(pool: web::Data<Pool>) -> Result<impl Responder, MyError> {
    let mut conn = pool.get()
        .map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            MyError::DatabaseError(e.to_string())
        })?;

    let results = assets
        .load::<Asset>(&mut conn)
        .map_err(|e| {
            error!("Failed to load assets: {}", e);
            MyError::DieselError(e)
        })?;

    if results.is_empty() {
        error!("No assets found in database");
    } else {
        info!("Found {} assets", results.len());
    }

    Ok(web::Json(results))
}

#[derive(serde::Deserialize)]
pub struct BuyRequest {
    quantity: i32,
}

#[post("/api/assets/{id}/buy")]
pub async fn buy_asset(
    pool: web::Data<Pool>,
    asset_id: web::Path<i32>,
    buy_request: web::Json<BuyRequest>,
) -> Result<impl Responder, MyError> {
    let mut conn = pool.get()
        .map_err(|e| {
            error!("Failed to get DB connection: {}", e);
            MyError::DatabaseError(e.to_string())
        })?;

    let result = conn.transaction(|conn| {
        // Get current asset
        let asset: Asset = assets
            .find(asset_id.into_inner())
            .first(conn)
            .map_err(|e| {
                error!("Failed to find asset: {}", e);
                MyError::DieselError(e)
            })?;

        // Check stock
        if asset.stock < buy_request.quantity {
            error!("Insufficient stock: requested {}, available {}", buy_request.quantity, asset.stock);
            return Err(MyError::ValidationError("Insufficient stock".into()));
        }

        // Update stock and timestamp
        let now = Utc::now().naive_utc();
        let updated = diesel::update(assets.find(asset.id))
            .set((
                stock.eq(asset.stock - buy_request.quantity),
                updated_at.eq(now)
            ))
            .execute(conn)
            .map_err(|e| {
                error!("Failed to update asset: {}", e);
                MyError::DieselError(e)
            })?;

        if updated == 0 {
            error!("No rows were updated when buying asset {}", asset.id);
            return Err(MyError::DatabaseError("Failed to update asset".into()));
        }

        Ok(())
    });

    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Purchase successful"
        }))),
        Err(e) => {
            error!("Transaction failed: {:?}", e);
            Err(e)
        }
    }
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy"
    }))
}

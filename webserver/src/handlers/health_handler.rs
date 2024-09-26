use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::database::db::DbPool;


pub async fn health_check(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get();

    match conn {
        Ok(_) => {
            HttpResponse::Ok().json(json!({"status": "healthy"}))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({"status": "unhealthy"}))
        }
    }
}

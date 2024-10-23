use crate::database::db::DbPool;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn health_check(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get();

    match conn {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "healthy"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "unhealthy"})),
    }
}

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

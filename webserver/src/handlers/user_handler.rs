use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use crate::auth::auth_middleware;
use crate::models::user::UserSub;

#[get("/info")]
async fn me(user_sub: UserSub) -> impl Responder {
    HttpResponse::Ok().json(json!({ "user_sub": user_sub}))
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").wrap(auth_middleware::Auth).service(me));
}

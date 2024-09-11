use crate::{auth::auth_middleware::UserSub, db::DbPool, services::user_service};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;


#[get("/info")]
async fn me(user_sub: UserSub) -> impl Responder {
    HttpResponse::Ok().json(json!({ "user_sub": user_sub}))
}

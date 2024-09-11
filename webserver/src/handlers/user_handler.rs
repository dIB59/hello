use crate::{auth::auth_middleware::UserSub, db::DbPool, services::user_service};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("")]
pub async fn register(
    pool: web::Data<DbPool>,
    credentials: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match user_service::register_user(
        &mut conn,
        &credentials.username,
        &credentials.password,
        &credentials.email,
    )
    .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(error) => {
            log::error!("Failed to create new User: {:?}", error);
            HttpResponse::InternalServerError().json("Error creating new User")
        }
    }
}

#[get("/info")]
async fn me(user_sub: UserSub) -> impl Responder {
    HttpResponse::Ok().json(json!({ "user_sub": user_sub}))
}

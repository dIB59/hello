use crate::{db::DbPool, services::user_service};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

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

#[get("/me")]
pub async fn me() -> impl Responder {
    HttpResponse::Ok().json("me")
}

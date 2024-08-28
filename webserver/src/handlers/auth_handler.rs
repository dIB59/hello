use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{auth::jwt_auth_service::create_jwt, database::db::DbPool, services::user_service};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match user_service::login(&mut conn, &credentials.email, &credentials.password).await {
        Ok(user) => {
            let bearer_token = create_jwt(&user.email);
            HttpResponse::Ok()
                .append_header(("Authorization", bearer_token))
                .json(user)
        }
        Err(error) => {
            log::error!("Failed to login User: {:?}", error);
            HttpResponse::Unauthorized().json("Invalid username or password")
        }
    }
}

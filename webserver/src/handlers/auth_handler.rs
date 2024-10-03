use actix_web::{HttpResponse, post, Responder, web};
use serde::Deserialize;

use crate::{auth::jwt_auth_service::create_jwt, database::db::DbPool, services::user_service};
use crate::handlers::auth_handler;
use crate::models::user::UserResponse;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match user_service::login(&mut conn, &credentials.email, &credentials.password) {
        Ok(user) => {
            let bearer_token = create_jwt(&user.email);
            let public_user: UserResponse = user.into();

            //add bearer token to response header
            HttpResponse::Ok()
                .append_header(("Authorization", format!("Bearer {}", bearer_token)))
                .json(public_user)
        }
        Err(error) => {
            log::error!("Failed to login User: {:?}", error);
            HttpResponse::Unauthorized().json("Invalid username or password")
        }
    }
}

#[post("/register")]
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
        Ok(user) => {
            let public_user: UserResponse = user.into();
            HttpResponse::Created().json(public_user)
        }
        Err(error) => {
            log::error!("Failed to create new User: {:?}", error);
            HttpResponse::InternalServerError().json("Error creating new User")
        }
    }
}


pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth")
        .service(login)
        .service(register));
}

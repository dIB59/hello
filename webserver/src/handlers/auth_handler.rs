use actix_web::{HttpResponse, post, Responder, ResponseError, web};
use serde::{Deserialize, Serialize};

use crate::{auth::jwt_auth_service::create_jwt, database::db::DbPool, services::user_service};
use crate::auth::error::AuthError;
use crate::database::error::DatabaseError;
use crate::handlers::auth_handler;
use crate::handlers::error::ApiError;
use crate::models::user::UserResponse;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<LoginRequest>,
) -> Result<impl Responder, impl ResponseError>
{
    let mut conn = pool.get().map_err(DatabaseError::from)?;

    let user = user_service::login(&mut conn, &credentials.email, &credentials.password)
        .map_err(AuthError::from)?;

    let bearer_token = create_jwt(&user.email);
    let public_user: UserResponse = user.into();

    Ok::<HttpResponse, ApiError>(
        HttpResponse::Ok()
        .append_header(("Authorization", format!("Bearer {}", bearer_token)))
        .json(public_user)
    )
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


mod tests {
    use actix_web::{App, test};
    use crate::database::db;
    use crate::database::test_db::TestDb;
    use crate::services::user_service::register_user;

    use super::*;

    #[actix_rt::test]
    async fn test_login_with_correct_credentials() {
        use actix_web::http::StatusCode;

        let db = TestDb::new();
        let pool = db::establish_connection(&db.url());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(auth_routes)
        ).await;

        let register_request = RegisterRequest {
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&register_request)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_request)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_login_with_incorrect_credentials() {
        use actix_web::http::StatusCode;

        let db = TestDb::new();
        let pool = db::establish_connection(&db.url());

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(auth_routes)
        ).await;

        let register_request = RegisterRequest {
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(&register_request)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CREATED);

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(&login_request)
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
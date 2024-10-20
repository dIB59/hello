use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    TokenGenerationError,
    #[error("Token validation error")]
    TokenValidationError,
    #[error("Token expired")]
    TokenExpired,
    #[error("Internal server error")]
    InternalServerError,
}

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match *self {
            AuthError::InvalidCredentials => {
                log::error!("Invalid credentials");
                actix_web::HttpResponse::Unauthorized().json("Invalid credentials")
            }
            AuthError::TokenGenerationError => {
                log::error!("Token generation error");
                actix_web::HttpResponse::InternalServerError().json("Token generation error")
            }
            AuthError::TokenValidationError => {
                log::info!("Token validation error");
                actix_web::HttpResponse::Unauthorized().json("Token validation error")
            }
            AuthError::TokenExpired => {
                log::info!("Token expired");
                actix_web::HttpResponse::Unauthorized().json("Token expired")
            }
            AuthError::InternalServerError => {
                log::error!("Internal server error");
                actix_web::HttpResponse::InternalServerError().json("Internal server error")
            }
        }
    }
}

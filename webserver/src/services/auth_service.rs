use actix_web::cookie::{time::Duration as ActixWebDuration, Cookie};
use core::fmt;
use std::collections::HashSet;
use std::future::{ready, Ready};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::user::User;
use crate::AppState;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use alcoholic_jwt::{validate, JWK, JWKS};
use jsonwebtoken::{decode, Algorithm};
use jsonwebtoken::{
    decode_header, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user_id: i32,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<Claims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id = claims.sub.as_str().parse::<i32>().expect("Invalid user id");
        req.extensions_mut().insert::<i32>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}

pub fn create_token(user: &User) -> Result<std::string::String, jsonwebtoken::errors::Error> {
    // let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    // let audience = std::env::var("AUDIENCE").expect("AUDIENCE must be set");
    let key = std::env::var("JWT_SECRET").expect("KEY must be set");
    let key = EncodingKey::from_secret(key.as_ref());
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let claims = Claims {
        sub: user.id.to_string(),
        role: user.email.to_string(),
        exp: now as usize + 3600,
    };
    let token = encode(&Header::default(), &claims, &key)?;
    Ok(token)
}

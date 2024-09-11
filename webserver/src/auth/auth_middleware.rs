use std::{
    future::{Future, ready, Ready},
    pin::Pin,
};

use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    error::{self},
    Error, FromRequest, HttpMessage, HttpRequest,
};
use serde::Serialize;
use serde_json::json;

use crate::models::user::UserSub;

use super::jwt_auth_service::{self};

// Auth middleware
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split_whitespace().nth(1))
            .unwrap_or("");

        let claims = jwt_auth_service::decode_jwt(token);

        match claims {
            Ok(claims) => {
                let user_id = claims.sub;
                log::info!("User ID: {}", user_id);
                req.extensions_mut().insert::<UserSub>(UserSub(user_id));
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                let json_error_message = json!({
                    "message": e.to_string(),
                });
                let error: Error = error::ErrorUnauthorized(json_error_message);
                Box::pin(async move { Err(error) })
            }
        }
    }
}

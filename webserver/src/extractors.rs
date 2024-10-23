use std::future::{ready, Ready};

use actix_web::{
    dev::{Payload, Service, Transform},
    Error, FromRequest, HttpMessage, HttpRequest,
};

use crate::models::user::UserSub;

impl FromRequest for UserSub {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let binding = req.extensions();
        let user_sub = binding.get::<UserSub>();

        match user_sub {
            Some(user_sub) => ready(Ok(user_sub.to_owned())),
            None => ready(Err(Error::from(actix_web::error::ErrorBadRequest(
                "No user sub found",
            )))),
        }
    }
}

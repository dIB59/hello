use std::future::{ready, Ready};

use actix_web::{
    dev::{Payload, Service, Transform}
    ,
    Error, FromRequest, HttpMessage, HttpRequest,
};

use crate::models::user::UserSub;

impl FromRequest for UserSub {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let user_sub = req
            .extensions()
            .get::<UserSub>()
            .expect("This should not happen")
            .to_owned();
        ready(Ok(user_sub))
    }
}

pub mod chat;
pub mod chat_server;

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, error::ErrorUnauthorized, error::ErrorBadRequest};
use actix_web_actors::ws;
use crate::auth::jwt_auth_service::decode_jwt;

pub async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}


pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Some(token) = auth_header.to_str().expect("Invalid UTF-8 Auth header").strip_prefix("Bearer "){
            match decode_jwt(&token) {
                Ok(claims) => ws::start(
                    chat::WsChatSession {
                            id: 0,
                            hb: Instant::now(),
                            room: "main".to_owned(),
                            name: None,
                            addr: srv.get_ref().clone(),
                        },
                        &req,
                        stream,
                    ),
                Err(error) => {
                    log::error!("Failed to login User: {:?}", error);
                    Err(ErrorUnauthorized("Unauthorized access"))
                }
            }
            }else{
                log::error!("Invalid token in header!");
                Err(ErrorBadRequest("Invalid token format. Expected 'Bearer <token>'"))
            }
        }
        else{
            log::error!("Authorization header is missing");
            Err(ErrorUnauthorized("Authorization header is missing"))
        }
}

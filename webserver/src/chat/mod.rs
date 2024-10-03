use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use actix::Addr;
use actix_web::{Error, HttpRequest, HttpResponse, Responder, web};
use actix_web_actors::ws;

pub mod chat_routes;
pub mod chat;
pub mod chat_server;

pub async fn get_count(count: web::Data<AtomicUsize>) -> impl Responder {
    let current_count = count.load(Ordering::SeqCst);
    format!("Visitors: {current_count}")
}

pub async fn chat_handler(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        chat::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use actix::Addr;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse, Responder, web};
use actix_web_actors::ws;

use crate::{database::db::DbPool, services::user_service::get_user_by_email};

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
    let user_id = req.extensions().get::<String>().unwrap_or(&"".to_string()).to_owned();
    let mut conn =
        req.app_data::<web::Data<DbPool>>().unwrap().get().expect("Failed to get DB connection.");

    match get_user_by_email(&user_id, &mut conn) {
        Ok(user) => {
            ws::start(
                chat::WsChatSession {
                    id: 0,
                    hb: Instant::now(),
                    room: "main".to_owned(),
                    name: Some(user.username),
                    addr: srv.get_ref().clone(),
                },
                &req,
                stream,
            )
        }
        Err(error) => {
            log::error!("Failed to login User: {:?}", error);
            Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
        }
    }
}

use crate::auth::auth_middleware;
use actix_web::web;

use super::chat_handler;

pub fn chat_route_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ws")
            .wrap(auth_middleware::Auth)
            .route("/chat", web::get().to(chat_handler)),
    );
}

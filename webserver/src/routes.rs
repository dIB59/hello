use actix_web::web;
use task_handler::task_routes;
use crate::routes::health_handler::health_check;

use crate::{
    auth::auth_middleware,
    chat::{chat_route, get_count},
    handlers::*,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(task_routes)
            .configure(protect)
            .configure(user_routes)
            .configure(auth_routes)
            .route("/health", web::get().to(health_check))
            .route("/count", web::get().to(get_count)),
    );
    cfg.service(
        web::scope("/ws")
            .wrap(auth_middleware::Auth)
            .route("/chat", web::get().to(chat_route)));
}

fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(user_handler::register));
}

fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(auth_handler::login));
}

fn protect(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/protected")
            .wrap(auth_middleware::Auth)
            .service(user_handler::me),
    );
}

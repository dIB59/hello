use actix_web::web;

use task_handler::task_routes;
use user_handler::user_routes;

use crate::{
    chat::{chat_routes::chat_route_auth, get_count},
    handlers::*,
};
use crate::auth::auth_middleware;
use crate::chat::chat_handler;
use crate::handlers::project_handler::project_routes;
use crate::routes::health_handler::health_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(task_routes)
            .configure(user_routes)
            .configure(project_routes)
            .configure(chat_route_auth)
            .configure(health_routes)
            .route("/count", web::get().to(get_count)),
    );
    cfg.service(
        web::scope("/ws")
            .wrap(auth_middleware::Auth)
            .route("/chat", web::get().to(chat_handler)));
}



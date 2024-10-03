use actix_web::web;

use auth_handler::auth_routes;
use task_handler::task_routes;
use user_handler::user_routes;

use crate::{
    chat::{chat_routes::chat_route_auth, get_count},
    handlers::*,
};
use crate::handlers::project_handler::project_routes;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(task_routes)
            .configure(user_routes)
            .configure(auth_routes)
            .configure(project_routes)
            .configure(chat_route_auth)
            .route("/count", web::get().to(get_count)),
    );
    // cfg.service(web::scope("/ws").wrap(auth_middleware::Auth).route("/chat", web::get().to(chat_handler)));
}

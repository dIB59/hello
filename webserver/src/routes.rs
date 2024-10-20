use actix_web::web;

use job_handler::job_routes_auth;
use task_handler::task_routes;
use user_handler::user_routes;

use crate::auth::auth_middleware;
use crate::chat::chat_handler;
use crate::handlers::auth_handler::auth_routes;
use crate::handlers::project_handler::project_routes;
use crate::routes::health_handler::health_routes;
use crate::{
    chat::{chat_routes::chat_route_auth, get_count},
    handlers::*,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(auth_routes)
            .configure(task_routes)
            .configure(user_routes)
            .configure(project_routes)
            .configure(chat_route_auth)
            .configure(job_routes_auth)
            .configure(health_routes)
            .route("/count", web::get().to(get_count)),
    );
    cfg.service(
        web::scope("/ws")
            .wrap(auth_middleware::Auth)
            .route("/chat", web::get().to(chat_handler)),
    );
}

use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Deserialize;

use crate::auth::auth_middleware;
use crate::database::db::DbPool;
use crate::models::user::UserSub;
use crate::services::project_service;
use crate::services::user_service::get_user_id_by_email;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: String,
}

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/projects")
        .wrap(auth_middleware::Auth)
        .service(create_project));
}

#[post("")]
pub async fn create_project(
    pool: web::Data<DbPool>,
    project: web::Json<CreateProjectRequest>,
    user_sub: UserSub,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    let id: i32 = get_user_id_by_email(&user_sub.0, &mut conn).expect("Failed to get user id");

    match project_service::create_project(&mut conn, &project.title, &project.description, &id)
    {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().json("Error creating new project"),
    }
}

#[get("")]
pub async fn get_projects(pool: web::Data<DbPool>, user_sub: UserSub) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    let id: i32 = get_user_id_by_email(&user_sub.0, &mut conn).expect("Failed to get user id");

    match project_service::get_projects(&mut conn, &id) {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().json("Error getting projects"),
    }
}

#[get("/{id}")]
pub async fn get_project(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match project_service::get_project_by_id(&mut conn, &id.into_inner()) {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::InternalServerError().json("Error getting project"),
    }
}

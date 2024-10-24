use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;

use crate::auth::auth_middleware;
use crate::database::db::DbPool;
use crate::database::error::DatabaseError;
use crate::handlers::error::ApiError;
use crate::models::user::UserSub;
use crate::run_async_query;
use crate::services::project_service;
use crate::services::user_service::get_user_id_by_email;

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub title: String,
    pub description: String,
}

pub fn project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .wrap(auth_middleware::Auth)
            .service(create_project),
    );
}

#[post("")]
pub async fn create_project(
    pool: web::Data<DbPool>,
    project: web::Json<CreateProjectRequest>,
    user_sub: UserSub,
) -> Result<impl Responder, impl ResponseError> {
    let created_project = run_async_query!(pool, |conn: &mut diesel::PgConnection| {
        let id: i32 = get_user_id_by_email(&user_sub.0, conn).map_err(DatabaseError::from)?;
        project_service::create_project(conn, &project.title, &project.description, &id)
            .map_err(DatabaseError::from)
    })?;

    Ok::<HttpResponse, ApiError>(HttpResponse::Created().json(created_project))
}

#[get("")]
pub async fn get_projects(
    pool: web::Data<DbPool>,
    user_sub: UserSub,
) -> Result<impl Responder, impl ResponseError> {
    let projects = run_async_query!(pool, |conn: &mut diesel::PgConnection| {
        let id: i32 = get_user_id_by_email(&user_sub.0, conn).map_err(DatabaseError::from)?;
        project_service::get_projects(conn, &id).map_err(DatabaseError::from)
    })?;

    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(projects))
}

#[get("/{id}")]
pub async fn get_project(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<impl Responder, impl ResponseError> {
    let user = run_async_query!(pool, |conn: &mut diesel::PgConnection| {
        project_service::get_project_by_id(conn, &id.into_inner()).map_err(DatabaseError::from)
    })?;
    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(user))
}

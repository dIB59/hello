use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Deserialize;

use crate::{auth::auth_middleware, db::DbPool};
use crate::models::user::UserSub;
use crate::services::task_service;
use crate::services::user_service::get_user_id_by_email;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    description: String,
    reward: i64,
    project_id: i32,
}

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .wrap(auth_middleware::Auth)
            .service(get_tasks)
            .service(get_task_by_id)
            .service(create_task),
    );
}

#[post("")]
pub async fn create_task(
    pool: web::Data<DbPool>,
    task: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match task_service::create_task(&mut conn, &task.description, task.reward, task.project_id) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().json("Error creating new task"),
    }
}

#[get("")]
pub async fn get_tasks(pool: web::Data<DbPool>, user_sub: UserSub) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    let users_id = get_user_id_by_email(&user_sub.0, &mut conn).expect("Failed to get user id");
    match task_service::get_tasks(&mut conn, &users_id) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().json("Error getting tasks"),
    }
}

#[get("/{id}")]
pub async fn get_task_by_id(pool: web::Data<DbPool>, id: web::Path<i32>, user_sub: UserSub) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    let users_id = get_user_id_by_email(&user_sub.0, &mut conn).expect("Failed to get user id");
    match task_service::get_task_by_id(&mut conn, id.into_inner(), &users_id) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().json("Error getting task"),
    }
}

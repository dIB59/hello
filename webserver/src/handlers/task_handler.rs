use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Deserialize;

use crate::{auth::auth_middleware, db::DbPool};
use crate::services::task_service;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    description: String,
    reward: i64,
    project_id: i32,
}

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .service(get_tasks)
            // .service(get_task_by_id)
            .wrap(auth_middleware::Auth)
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
pub async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    match task_service::get_tasks(&mut conn) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().json("Error getting tasks"),
    }
}

// #[get("/{id}")]
// pub async fn get_task_by_id(pool: web::Data<DbPool>, id: web::Path<i32>) -> impl Responder {
//     let mut conn = pool.get().expect("Failed to get DB connection.");
//     match task_service::get_task_by_id(&mut conn, id.into_inner()).await {
//         Ok(task) => HttpResponse::Ok().json(task),
//         Err(_) => HttpResponse::InternalServerError().json("Error getting task"),
//     }
// }

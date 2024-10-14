use actix_web::{get, HttpResponse, post, Responder, web};
use serde::Deserialize;

use crate::get_db_connection_async;
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
            .service(get_tasks)
            .service(create_task)
    );
}

#[post("")]
pub async fn create_task(
    pool: web::Data<DbPool>,
    task: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let user = get_db_connection_async!(pool,|conn| {task_service::create_task(conn, &task.description, task.reward, task.project_id)});
    match user {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().json("Error creating new task"),
    }
}

#[get("")]
pub async fn get_tasks(pool: web::Data<DbPool>, user_sub: UserSub) -> impl Responder {

    let user = get_db_connection_async!(pool, |conn: &mut diesel::PgConnection| {
        // First, get the user ID by email
        let users_id = get_user_id_by_email(&user_sub.0, conn)
            .expect("Failed to get user id");
        
        // Then, retrieve tasks using the user ID
        task_service::get_tasks(conn, &users_id)
    });
    match user {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().json("Error getting tasks"),
    }
}

#[get("/{id}")]
pub async fn get_task_by_id(pool: web::Data<DbPool>, id: web::Path<i32>, user_sub: UserSub) -> impl Responder {
    let user = get_db_connection_async!(pool, |conn: &mut diesel::PgConnection| {
        let users_id = get_user_id_by_email(&user_sub.0, conn).expect("Failed to get user id");
        task_service::get_task_by_id(conn, id.into_inner(), &users_id)
    });
    match user {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::InternalServerError().json("Error getting task"),
    }
}


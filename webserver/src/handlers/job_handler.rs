use crate::{
    auth::auth_middleware,
    database::db::DbPool,
    handlers::error::ApiError,
    models::{job::NewJob, user::UserSub},
    run_async_query,
    services::{job_service, user_service::get_user_id_by_email},
};
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub job_title: String,
    pub company_name: String,
    pub company_logo: Option<String>,
    pub company_location: String,
    pub company_ranking: i32,
    pub employment_type: String,
    pub time_schedule: String,
    pub workplace_type: String,
    pub department: String,
    pub job_description: String,
    pub responsabilities: String,
    pub qualifications: String,
    pub required_skills: Vec<String>,
    pub preferred_skills: Vec<String>,
    pub experience_level: String,
    pub min_salary: i64,
    pub max_salary: i64,
    pub comp_structure: String,
    pub currency: String,
    pub benefits_and_perks: String,
    pub work_hours_flexibile: bool,
    pub apply_through_platform: bool,
    pub external_url: Option<String>,
    pub email: Option<String>,
    pub audience_type: String,
    pub target_candidates: String,
    pub candidate_recommendations: bool,
    pub job_screening_questions: Option<Vec<String>>,
}
/*impl From<(&i32, CreateJobRequest)> for NewJob {
    fn from(value: (&i32, CreateJobRequest)) -> Self {
        let request = &value.1;
        let user_id = value.0;
        NewJob {
            user_id,
            job_title: &request.job_title,
            company_name: &request.company_name,
            company_logo: request.company_logo.as_deref(),
            company_location: &request.company_location,
            company_ranking: &request.company_ranking,
            employment_type: &request.employment_type,
            time_schedule: &request.time_schedule,
            workplace_type: &request.workplace_type,
            department: &request.department,
            job_description: &request.job_description,
            responsibilities: &request.responsibilities,
            qualifications: &request.qualifications,
            required_skills: &request.required_skills,
            preferred_skills: &request.preferred_skills,
            experience_level: &request.experience_level,
            min_salary: &request.min_salary,
            max_salary: &request.max_salary,
            comp_structure: &request.comp_structure,
            currency: &request.currency,
            benefits_and_perks: &request.benefits_and_perks,
            work_hours_flexible: request.work_hours_flexible,
            apply_through_platform: request.apply_through_platform,
            external_url: request.external_url.as_deref(),
            email: request.email.as_deref(),
            audience_type: &request.audience_type,
            target_candidates: &request.target_candidates,
            candidate_recommendations: request.candidate_recommendations,
            jobs_screening_questions: request.job_screening_questions.as_deref(),
        }
    }
}*/
// helper method to borrow 'job' parameters from the incoming request into a NewJob struct
impl CreateJobRequest {
    fn copy_request<'a>(&'a self, logged_user_id: &'a i32) -> NewJob<'a> {
        let new_job: NewJob<'a> = NewJob {
            user_id: &logged_user_id,
            job_title: &self.job_title,
            company_name: &self.company_name,
            company_logo: Option::as_ref(&self.company_logo).unwrap(),
            company_location: &self.company_location,
            company_ranking: &self.company_ranking,
            employment_type: &self.employment_type,
            time_schedule: &self.time_schedule,
            workplace_type: &self.workplace_type,
            department: &self.department,
            job_description: &self.job_description,
            responsabilities: &self.responsabilities,
            qualifications: &self.qualifications,
            required_skills: &self.required_skills,
            preferred_skills: &self.preferred_skills,
            experience_level: &self.experience_level,
            min_salary: &self.min_salary,
            max_salary: &self.max_salary,
            comp_structure: &self.comp_structure,
            currency: &self.currency,
            benefits_and_perks: &self.benefits_and_perks,
            work_hours_flexibile: self.work_hours_flexibile,
            apply_through_platform: self.apply_through_platform,
            external_url: Option::as_ref(&self.external_url).unwrap(),
            email: Option::as_ref(&self.email).unwrap(),
            audience_type: &self.audience_type,
            target_candidates: &self.target_candidates,
            candidate_recommendations: self.candidate_recommendations,
            jobs_screening_questions: Option::as_ref(&self.job_screening_questions).unwrap(),
        };
        new_job
    }
}

#[post("")]
pub async fn create_job(
    user_sub: UserSub,
    pool: web::Data<DbPool>,
    job_req: web::Json<CreateJobRequest>,
) -> Result<impl Responder, impl ResponseError> {
    // let user = web::block(move || {
    //             let mut conn = pool.clone().get().expect("Failed to get DB connection.");
    //             let user_id = get_user_id_by_email(&user_sub.0, &mut conn).expect("Failed to get user id");
    //             let new_job : NewJob = job_req.copy_request(&user_id);
    //             job_service::create_job(&mut conn, &new_job)
    //         })
    //         .await
    //         .map_err(actix_web::error::ErrorInternalServerError).expect("internal server error");
    let user = run_async_query!(pool, |conn: &mut diesel::PgConnection| {
        let user_id = get_user_id_by_email(&user_sub.0, conn).expect("Failed to get user id");
        let new_job: NewJob = job_req.copy_request(&user_id);
        job_service::create_job(conn, &new_job).map_err(DatabaseError::from)
    })?;
    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(user))
}

#[get("")]
pub async fn get_jobs(pool: web::Data<DbPool>) -> Result<impl Responder, impl ResponseError> {
    let user = run_async_query!(pool, |conn| {
        job_service::get_jobs(conn).map_err(DatabaseError::from)
    })?;

    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(user))
}

#[get("/mine")]
pub async fn get_my_jobs(
    user_sub: UserSub,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, impl ResponseError> {
    let user = run_async_query!(pool, |conn: &mut diesel::PgConnection| {
        let user_id = get_user_id_by_email(&user_sub.0, conn).expect("Failed to get user id");
        job_service::get_my_jobs(conn, user_id).map_err(DatabaseError::from)
    })?;
    Ok::<HttpResponse, ApiError>(HttpResponse::Ok().json(user))
}

pub fn job_routes_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jobs")
            .wrap(auth_middleware::Auth)
            .service(get_jobs)
            .service(create_job)
            .service(get_my_jobs),
    );
}

use crate::schema::jobs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = jobs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Job {
    pub id: i32,
    pub user_id: i32,
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
    pub required_skills: Vec<Option<String>>,
    pub preferred_skills: Vec<Option<String>>,
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
    pub jobs_screening_questions: Option<Vec<Option<String>>>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = jobs)]
pub struct NewJob<'a> {
    pub user_id: &'a i32,
    pub job_title: &'a str,
    pub company_name: &'a str,
    pub company_logo: &'a str,
    pub company_location: &'a str,
    pub company_ranking: &'a i32,
    pub employment_type: &'a str,
    pub time_schedule: &'a str,
    pub workplace_type: &'a str,
    pub department: &'a str,
    pub job_description: &'a str,
    pub responsabilities: &'a str,
    pub qualifications: &'a str,
    pub required_skills: &'a Vec<String>,
    pub preferred_skills: &'a Vec<String>,
    pub experience_level: &'a str,
    pub min_salary: &'a i64,
    pub max_salary: &'a i64,
    pub comp_structure: &'a str,
    pub currency: &'a str,
    pub benefits_and_perks: &'a str,
    pub work_hours_flexibile: bool,
    pub apply_through_platform: bool,
    pub external_url: &'a str,
    pub email: &'a str,
    pub audience_type: &'a str,
    pub target_candidates: &'a str,
    pub jobs_screening_questions: &'a Vec<String>,
    pub candidate_recommendations: bool,
}

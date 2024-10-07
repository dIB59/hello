use crate::models::job::{Job, NewJob};
use crate::schema::{self, jobs};
use diesel::prelude::*;
use diesel::result::Error;

pub async fn create_job<'a>(
    conn: &mut PgConnection,
    new_job: & NewJob<'a>,
) -> Result<Job, Error> {
    println!("{:?}",new_job);
    let some = diesel::insert_into(jobs::table)
        .values(new_job)
        .returning(Job::as_returning())
        .get_result(conn);
    log::info!("{:?}", some);
    return some;
}

pub(crate) async fn get_jobs(conn: &mut PgConnection) -> Result<Vec<Job>, Error> {
    let jobs = jobs::table.load::<Job>(conn);
    return jobs;
}

pub(crate) async fn get_my_jobs(conn: &mut PgConnection, user_id : i32) -> Result<Vec<Job>, Error> {
    let jobs = jobs::table
                                        .filter(schema::jobs::user_id.eq(user_id))
                                        .load::<Job>(conn);
    return jobs;
}


#[cfg(test)]
mod tests {
    use crate::database::test_db::TestDb;
    use crate::services::user_service::register_user;

    use super::*;

    fn create_test_job<'a>(user_id :&'a i32,
                required_skills_vec : &'a Vec<String>,
                preferred_skills_vec : &'a Vec<String>,
                jobs_screening_vec: &'a Vec<String>  ) ->  NewJob<'a>{
        let job:NewJob<'a> = NewJob{
            user_id : &user_id,
            job_title : "worker",
            company_name : "company",
            company_logo : "logo",
            company_location : "location",
            company_ranking : &1,
            employment_type : "temp",
            time_schedule : "schedule",
            workplace_type : "remote",
            department : "IT",
            job_description : "Need IT Worker to do stuff",
            responsabilities : "do stuff in IT",
            qualifications : "Bsc in IT",
            required_skills : &required_skills_vec,
            preferred_skills : &preferred_skills_vec,
            experience_level : "Senior",
            min_salary : &20,
            max_salary : &40,
            comp_structure : "monthly",
            currency : "SEK",
            benefits_and_perks : "gym membership",
            work_hours_flexibile : true,
            apply_through_platform : false,
            external_url : "www.companywebsite.com",
            email: "",
            audience_type : "All",
            target_candidates : "All",
            candidate_recommendations : true,
            jobs_screening_questions: &jobs_screening_vec
        };
        job
    }
    #[actix_rt::test]
    async fn test_create_job_success() {
        let db = TestDb::new();
        
        let user_id = register_user(&mut db.conn(), "test job", "testpassword", "test@test.com")
        .await.expect("Failed to register user")
        .id;

        let required_skills_vec = &vec!["tall".to_string()];
        let preferred_skills_vec = &vec!["not short".to_string()];
        let jobs_screening_vec:&Vec<String> =&vec!["what is your name?".to_string(),"where do you live?".to_string()];
        let job = create_test_job(&user_id,
                                            required_skills_vec,
                                            preferred_skills_vec,
                                            jobs_screening_vec);
        
        let result = create_job(&mut db.conn(), &job)
            .await;

        assert!(
            result.is_ok(),
            "Job creation failed when it should have succeeded"
        );

        let created_job = result.unwrap();
        assert_eq!(created_job.user_id , *job.user_id);
        assert_eq!(created_job.job_title , job.job_title);
        assert_eq!(created_job.company_name , job.company_name);
        assert_eq!(created_job.company_logo.unwrap_or("missing".to_string()) , job.company_logo);
        assert_eq!(created_job.company_location , job.company_location);
        assert_eq!(created_job.company_ranking , *job.company_ranking);
        assert_eq!(created_job.employment_type , job.employment_type);
        assert_eq!(created_job.time_schedule , job.time_schedule);
        assert_eq!(created_job.workplace_type , job.workplace_type);
        assert_eq!(created_job.department , job.department);
        assert_eq!(created_job.job_description , job.job_description);
        assert_eq!(created_job.responsabilities , job.responsabilities);
        assert_eq!(created_job.qualifications , job.qualifications);
        assert_eq!(created_job.required_skills.into_iter()
                    .map(|opt| opt.unwrap_or_else(|| "missing".to_string()))
                    .collect::<Vec<String>>() , *job.required_skills);
        assert_eq!(created_job.preferred_skills.into_iter()
                    .map(|opt| opt.unwrap_or_else(|| "missing".to_string()))
                    .collect::<Vec<String>>() , *job.preferred_skills);
        assert_eq!(created_job.experience_level , job.experience_level);
        assert_eq!(created_job.min_salary , *job.min_salary);
        assert_eq!(created_job.max_salary , *job.max_salary);
        assert_eq!(created_job.comp_structure , job.comp_structure);
        assert_eq!(created_job.currency , job.currency);
        assert_eq!(created_job.benefits_and_perks , job.benefits_and_perks);
        assert_eq!(created_job.work_hours_flexibile , job.work_hours_flexibile);
        assert_eq!(created_job.apply_through_platform , job.apply_through_platform);
        assert_eq!(created_job.external_url.unwrap_or("missing".to_string()) , job.external_url);
        assert_eq!(created_job.email.unwrap_or("missing".to_string()), job.email);
        assert_eq!(created_job.audience_type , job.audience_type);
        assert_eq!(created_job.target_candidates , job.target_candidates);
        assert_eq!(created_job.candidate_recommendations , job.candidate_recommendations);
        assert_eq!(created_job.jobs_screening_questions.unwrap_or(vec![]).into_iter()
                    .map(|opt| opt.unwrap_or_else(|| "missing".to_string()))
                    .collect::<Vec<String>>() , *job.jobs_screening_questions.clone());
    }
     
    #[actix_rt::test]
    async fn get_jobs_success() {
        let db = TestDb::new();
        
        let user_id = register_user(&mut db.conn(), "test job", "testpassword", "test@test.com")
        .await.expect("Failed to register user")
        .id;

        let required_skills_vec = &vec!["tall".to_string()];
        let preferred_skills_vec = &vec!["not short".to_string()];
        let jobs_screening_vec:&Vec<String> =&vec!["what is your name?".to_string(),"where do you live?".to_string()];
        let job = create_test_job(&user_id,
                                            required_skills_vec,
                                            preferred_skills_vec,
                                            jobs_screening_vec);
        
        let created_job_result = create_job(&mut db.conn(), &job)
            .await;
        
        let stored_jobs_result = get_jobs(&mut db.conn())
            .await;
        assert!(
            stored_jobs_result.is_ok(),
            "Job retrieval failed when it should have succeeded"
        );
        let stored_jobs_list = stored_jobs_result.unwrap();
        let last_stored_job = stored_jobs_list.last().unwrap();

        let created_job = created_job_result.unwrap();
        assert_eq!(created_job.user_id , last_stored_job.user_id);
        assert_eq!(created_job.job_title , last_stored_job.job_title);
        assert_eq!(created_job.company_name , last_stored_job.company_name);
        assert_eq!(created_job.company_logo , last_stored_job.company_logo);
        assert_eq!(created_job.company_location , last_stored_job.company_location);
        assert_eq!(created_job.company_ranking , last_stored_job.company_ranking);
        assert_eq!(created_job.employment_type , last_stored_job.employment_type);
        assert_eq!(created_job.time_schedule , last_stored_job.time_schedule);
        assert_eq!(created_job.workplace_type , last_stored_job.workplace_type);
        assert_eq!(created_job.department , last_stored_job.department);
        assert_eq!(created_job.job_description , last_stored_job.job_description);
        assert_eq!(created_job.responsabilities , last_stored_job.responsabilities);
        assert_eq!(created_job.qualifications , last_stored_job.qualifications);
        assert_eq!(created_job.required_skills, last_stored_job.required_skills);
        assert_eq!(created_job.preferred_skills, last_stored_job.preferred_skills);
        assert_eq!(created_job.experience_level , job.experience_level);
        assert_eq!(created_job.min_salary , last_stored_job.min_salary);
        assert_eq!(created_job.max_salary , last_stored_job.max_salary);
        assert_eq!(created_job.comp_structure , last_stored_job.comp_structure);
        assert_eq!(created_job.currency , last_stored_job.currency);
        assert_eq!(created_job.benefits_and_perks , last_stored_job.benefits_and_perks);
        assert_eq!(created_job.work_hours_flexibile , last_stored_job.work_hours_flexibile);
        assert_eq!(created_job.apply_through_platform , last_stored_job.apply_through_platform);
        assert_eq!(created_job.external_url, last_stored_job.external_url);
        assert_eq!(created_job.email, last_stored_job.email);
        assert_eq!(created_job.audience_type , last_stored_job.audience_type);
        assert_eq!(created_job.target_candidates , last_stored_job.target_candidates);
        assert_eq!(created_job.candidate_recommendations , last_stored_job.candidate_recommendations);
        assert_eq!(created_job.jobs_screening_questions , last_stored_job.jobs_screening_questions);
    }
}
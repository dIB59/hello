use crate::models::job::{Job, NewJob};
use crate::schema::{self, jobs};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_job<'a>(conn: &mut PgConnection, new_job: &NewJob<'a>) -> Result<Job, Error> {
    println!("{:?}", new_job);
    let some = diesel::insert_into(jobs::table)
        .values(new_job)
        .returning(Job::as_returning())
        .get_result(conn);
    log::info!("{:?}", some);
    return some;
}

pub(crate) fn get_jobs(conn: &mut PgConnection) -> Result<Vec<Job>, Error> {
    let jobs = jobs::table.load::<Job>(conn);
    return jobs;
}

pub(crate) fn get_my_jobs(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Job>, Error> {
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

    impl<'a> PartialEq<NewJob<'a>> for Job {
        fn eq(&self, other: &NewJob) -> bool {
            self.user_id == *other.user_id
                && self.company_logo.clone().unwrap_or("missing".to_string()) == other.company_logo
                && self.user_id == *other.user_id
                && self.job_title == other.job_title
                && self.company_name == other.company_name
                && self.company_logo.clone().unwrap_or("missing".to_string()) == other.company_logo
                && self.company_location == other.company_location
                && self.company_ranking == *other.company_ranking
                && self.employment_type == other.employment_type
                && self.time_schedule == other.time_schedule
                && self.workplace_type == other.workplace_type
                && self.department == other.department
                && self.job_description == other.job_description
                && self.responsabilities == other.responsabilities
                && self.qualifications == other.qualifications
                && self
                    .required_skills
                    .clone()
                    .into_iter()
                    .map(|opt| opt.unwrap_or_else(|| "missing".to_string()))
                    .collect::<Vec<String>>()
                    == *other.required_skills
                && self
                    .preferred_skills
                    .clone()
                    .into_iter()
                    .map(|opt| opt.unwrap_or_else(|| "missing".to_string()))
                    .collect::<Vec<String>>()
                    == *other.preferred_skills
                && self.experience_level == other.experience_level
                && self.min_salary == *other.min_salary
                && self.max_salary == *other.max_salary
                && self.comp_structure == other.comp_structure
                && self.currency == other.currency
                && self.benefits_and_perks == other.benefits_and_perks
                && self.work_hours_flexibile == other.work_hours_flexibile
                && self.apply_through_platform == other.apply_through_platform
                && self.external_url.clone().unwrap_or("missing".to_string()) == other.external_url
                && self.email.clone().unwrap_or("missing".to_string()) == other.email
                && self.audience_type == other.audience_type
                && self.target_candidates == other.target_candidates
                && self.candidate_recommendations == other.candidate_recommendations
                && self
                    .jobs_screening_questions
                    .clone()
                    .unwrap_or(vec![])
                    .into_iter()
                    .map(|opt| opt.unwrap_or_else(|| "missing".to_string()))
                    .collect::<Vec<String>>()
                    == *other.jobs_screening_questions.clone()
        }
    }

    fn create_test_job<'a>(
        user_id: &'a i32,
        required_skills_vec: &'a Vec<String>,
        preferred_skills_vec: &'a Vec<String>,
        jobs_screening_vec: &'a Vec<String>,
    ) -> NewJob<'a> {
        let job: NewJob<'a> = NewJob {
            user_id: &user_id,
            job_title: "worker",
            company_name: "company",
            company_logo: "logo",
            company_location: "location",
            company_ranking: &1,
            employment_type: "temp",
            time_schedule: "schedule",
            workplace_type: "remote",
            department: "IT",
            job_description: "Need IT Worker to do stuff",
            responsabilities: "do stuff in IT",
            qualifications: "Bsc in IT",
            required_skills: &required_skills_vec,
            preferred_skills: &preferred_skills_vec,
            experience_level: "Senior",
            min_salary: &20,
            max_salary: &40,
            comp_structure: "monthly",
            currency: "SEK",
            benefits_and_perks: "gym membership",
            work_hours_flexibile: true,
            apply_through_platform: false,
            external_url: "www.companywebsite.com",
            email: "",
            audience_type: "All",
            target_candidates: "All",
            candidate_recommendations: true,
            jobs_screening_questions: &jobs_screening_vec,
        };
        job
    }

    #[test]
    fn test_create_job_success() {
        let db = TestDb::new();

        let user_id = register_user(&mut db.conn(), "test job", "testpassword", "test@test.com")
            .expect("Failed to register user")
            .id;

        let required_skills_vec = &vec!["tall".to_string()];
        let preferred_skills_vec = &vec!["not short".to_string()];
        let jobs_screening_vec: &Vec<String> = &vec![
            "what is your name?".to_string(),
            "where do you live?".to_string(),
        ];
        let new_job = create_test_job(
            &user_id,
            required_skills_vec,
            preferred_skills_vec,
            jobs_screening_vec,
        );

        let result = create_job(&mut db.conn(), &new_job);

        assert!(
            result.is_ok(),
            "Job creation failed when it should have succeeded"
        );

        let created_job = result.unwrap();
        assert_eq!(created_job, new_job);
    }

    #[test]
    fn get_jobs_success() {
        let db = TestDb::new();

        let user_id = register_user(&mut db.conn(), "test job", "testpassword", "test@test.com")
            .expect("Failed to register user")
            .id;

        let required_skills_vec = &vec!["tall".to_string()];
        let preferred_skills_vec = &vec!["not short".to_string()];
        let jobs_screening_vec: &Vec<String> = &vec![
            "what is your name?".to_string(),
            "where do you live?".to_string(),
        ];
        let new_job = create_test_job(
            &user_id,
            required_skills_vec,
            preferred_skills_vec,
            jobs_screening_vec,
        );

        create_job(&mut db.conn(), &new_job)
            .expect("Job creation failed when it should have succeeded");

        let stored_jobs_result = get_jobs(&mut db.conn());
        assert!(
            stored_jobs_result.is_ok(),
            "Job retrieval failed when it should have succeeded"
        );
        let stored_jobs_list = stored_jobs_result.unwrap();
        let last_stored_job = stored_jobs_list.last().unwrap();

        assert_eq!(*last_stored_job, new_job);
    }
}

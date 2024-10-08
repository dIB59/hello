-- Your SQL goes here
CREATE TABLE "jobs"(
	"id" INT4 NOT NULL PRIMARY KEY,
    "user_id" INT4 NOT NULL,                -- refers to the user creating the job posting for access privileges
	"job_title" VARCHAR NOT NULL,
	"company_name" VARCHAR NOT NULL,
	"company_logo" VARCHAR,                 -- should include a link to company logo
    "company_location" VARCHAR NOT NULL,
    "company_ranking" INT4 NOT NULL,
    "employment_type" VARCHAR NOT NULL,
    "time_schedule" VARCHAR NOT NULL,
    "workplace_type" VARCHAR NOT NULL,
    "department" VARCHAR NOT NULL,
    "job_description" TEXT NOT NULL,
    "responsabilities" TEXT NOT NULL,
    "qualifications" TEXT NOT NULL,
    "required_skills" TEXT[] NOT NULL,
    "preferred_skills" TEXT[] NOT NULL,
    "experience_level" VARCHAR NOT NULL,
    "min_salary" INT8 NOT NULL,
    "max_salary" INT8 NOT NULL,
    "comp_structure" VARCHAR NOT NULL,
    "currency" VARCHAR NOT NULL,
    "benefits_and_perks" TEXT NOT NULL,
    "work_hours_flexibile" BOOL NOT NULL,
    "apply_through_platform" BOOL NOT NULL,
    "external_url" VARCHAR,
    "email" VARCHAR,
    "audience_type" VARCHAR NOT NULL,
    "target_candidates" VARCHAR NOT NULL,
    "candidate_recommendations" BOOL NOT NULL,
    "jobs_screening_questions" TEXT[],
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE SEQUENCE jobs_id_seq;
ALTER TABLE "jobs" ALTER COLUMN "id" SET DEFAULT nextval('jobs_id_seq');
SELECT setval('jobs_id_seq', (SELECT MAX(id) FROM users));

 
   
use diesel::result::Error;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::models::project::Project;
use crate::{models::project::NewProject, schema::projects};

pub fn create_project(
    conn: &mut PgConnection,
    title: &str,
    description: &str,
    user_id: &i32,
) -> Result<Project, Error> {
    let new_project = NewProject {
        title,
        description,
        user_id,
    };
    let project = diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(conn);
    log::info!("{:?}", project);
    return project;
}

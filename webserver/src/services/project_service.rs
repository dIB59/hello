use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::result::Error;

use crate::{models::project::NewProject, schema::projects};
use crate::models::project::Project;

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

pub fn get_projects(conn: &mut PgConnection, user: &i32) -> Result<Vec<Project>, Error> {
    let projects = projects::table
        .filter(projects::user_id.eq(user))
        .load::<Project>(conn);
    return projects;
}

pub fn get_project_by_id(conn: &mut PgConnection, project_id: &i32) -> Result<Project, Error> {
    let project = projects::table
        .find(project_id)
        .get_result(conn);
    return project;
}

mod tests {
    use crate::database::test_db::TestDb;
    use crate::services::user_service::register_user;

    use super::*;

    #[actix_rt::test]
    async fn test_create_project() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";

        let user = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .await.expect("Failed to register user");

        let result = create_project(&mut conn, title, description, &user.id);
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "Project creation failed when it should have succeeded"
        );

        let project = result.unwrap();
        assert_eq!(project.title, title);
        assert_eq!(project.description, description);
        assert_eq!(project.user_id, user.id);
    }

    #[actix_rt::test]
    async fn test_get_projects() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";
        let user_id = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .await.expect("Failed to register user").id;


        let result = create_project(&mut conn, title, description, &user_id);
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "Project creation failed when it should have succeeded"
        );

        let project = result.unwrap();
        let projects = get_projects(&mut conn, &user_id).unwrap();
        assert!(projects.contains(&project));
    }

    #[actix_rt::test]
    async fn test_get_project_by_id() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";
        let user_id = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .await.expect("Failed to register user").id;

        let result = create_project(&mut conn, title, description, &user_id);
        println!("{:?}", result);
        assert!(
            result.is_ok(),
            "Project creation failed when it should have succeeded"
        );

        let project = result.unwrap();
        let project_by_id = get_project_by_id(&mut conn, &project.id).unwrap();
        assert_eq!(project_by_id.title, title);
        assert_eq!(project_by_id.description, description);
        assert_eq!(project_by_id.user_id, user_id);
    }
}
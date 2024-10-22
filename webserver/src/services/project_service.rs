use diesel::result::Error;
use diesel::{
    ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper,
};

use crate::models::project::Project;
use crate::models::task::Task;
use crate::{models::project::NewProject, schema, schema::projects};

pub fn create_project(
    conn: &mut PgConnection,
    title: &str,
    description: &str,
    user_id: &i32,
) -> Result<Project, Error> {
    use crate::schema::tasks::dsl::tasks;
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

fn get_project_with_tasks(
    conn: &mut PgConnection,
    project_id: &i32,
) -> QueryResult<(Project, Vec<Task>)> {
    let project = projects::table.find(project_id).first::<Project>(conn)?;

    let project_tasks = schema::tasks::table
        .filter(schema::tasks::project_id.eq(project_id))
        .load::<Task>(conn)?;

    Ok((project, project_tasks))
}

pub fn get_project_by_id(conn: &mut PgConnection, project_id: &i32) -> Result<Project, Error> {
    let project = projects::table.find(project_id).get_result(conn);
    return project;
}

mod tests {
    use crate::database::test_db::TestDb;
    use crate::services::task_service::create_task;
    use crate::services::user_service::register_user;

    use super::*;

    #[test]
    fn test_create_project() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";

        let user = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .expect("Failed to register user");

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

    #[test]
    fn test_get_projects() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";
        let user_id = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .expect("Failed to register user")
            .id;

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

    #[test]
    fn test_get_project_by_id() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";
        let user_id = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .expect("Failed to register user")
            .id;

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

    #[test]
    fn test_get_project_with_tasks() {
        let db = TestDb::new();
        let mut conn = db.conn();

        let title = "Test Project";
        let description = "Test Project Description";
        let user_id = register_user(&mut conn, "testuser", "password123", "test@example.com")
            .expect("Failed to register user")
            .id;

        let project_id = create_project(&mut conn, title, description, &user_id)
            .expect("Failed to create project")
            .id;
        let (project_new, tasks_new) =
            get_project_with_tasks(&mut conn, &project_id).expect("Failed to get project");
        assert_eq!(project_new.id, project_id);
        assert_eq!(tasks_new.len(), 0);

        let task_1 = create_task(&mut conn, "test task 1", 100, project_id);
        let task_2 = create_task(&mut conn, "test task 2", 200, project_id);

        let (project, tasks) =
            get_project_with_tasks(&mut conn, &project_id).expect("Failed to get project");
        assert_eq!(tasks.len(), 2);
    }
}

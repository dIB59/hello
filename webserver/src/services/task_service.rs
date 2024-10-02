use diesel::prelude::*;
use diesel::result::Error;

use crate::models::task::{NewTask, Task};
use crate::schema::tasks;

pub fn create_task(
    conn: &mut PgConnection,
    description: &str,
    reward: i64,
) -> Result<Task, Error> {
    let new_task = NewTask {
        description,
        reward,
        completed: false,
    };
    let some = diesel::insert_into(tasks::table)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn);
    log::info!("{:?}", some);
    return some;
}

pub(crate) fn get_tasks(conn: &mut PgConnection) -> Result<Vec<Task>, Error> {
    tasks::table.load(conn)
}

pub(crate) fn get_task_by_id(conn: &mut PgConnection, task_id: i32) -> Result<Task, Error> {
    use crate::schema::tasks::dsl::*;
    let task = tasks.filter(id.eq(task_id))
        .first::<Task>(conn)?;
    Ok(task)
}

#[cfg(test)]
mod tests {
    use crate::database::test_db::TestDb;

    use super::*;

    #[actix_rt::test]
    async fn test_create_task_success() {
        let db = TestDb::new();

        let description = "test task";
        let reward = 100;

        let result = create_task(&mut db.conn(), description, reward);

        assert!(
            result.is_ok(),
            "Task creation failed when it should have succeeded"
        );

        let created_task = result.unwrap();
        assert_eq!(created_task.description, description);
        assert_eq!(created_task.reward, reward);
    }
}

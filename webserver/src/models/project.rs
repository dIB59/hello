use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::models::user::User;

#[derive(
    Associations, Identifiable, Selectable, Queryable, Debug, PartialEq, Eq, Clone, Serialize,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::projects)]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,

}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            user_id: project.user_id,
            title: project.title,
            description: project.description,
            created_at: project.created_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject<'a> {
    pub(crate) user_id: &'a i32,
    pub(crate) title: &'a str,
    pub(crate) description: &'a str,
}

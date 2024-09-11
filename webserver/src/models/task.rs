use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::tasks;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub reward: i64,
    pub completed: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub description: &'a str,
    pub reward: &'a i64,
    pub completed: bool,
}

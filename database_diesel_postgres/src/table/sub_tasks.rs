use chrono::naive::NaiveDateTime;

use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::sub_tasks;
use crate::table::tasks::Task;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Task)]
#[table_name = "sub_tasks"]
pub struct SubTask {
    pub id: i32,
    pub task_id: i32,
    pub title: String,
    pub done: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "sub_tasks"]
pub struct NewSubTask<'a> {
    pub task_id: i32,
    pub title: &'a str,
}

pub fn create_sub_task<'a>(conn: &PgConnection, task_id: i32, title: &'a str) -> SubTask {
    let new_sub_task = NewSubTask { task_id, title };

    diesel::insert_into(sub_tasks::table)
        .values(&new_sub_task)
        .get_result(conn)
        .expect("Error saving new task")
}

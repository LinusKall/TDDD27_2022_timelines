use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::tasks;
use crate::table::timelines::Timeline;
use chrono::naive::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Timeline)]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub done: bool,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewEvent<'a> {
    pub timeline_id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub end_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub timeline_id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub end_time: NaiveDateTime,
}

pub fn create_event<'a>(
    conn: &PgConnection,
    timeline_id: i32,
    title: &'a str,
    body: Option<&'a str>,
    end_time: NaiveDateTime,
) -> Task {
    let new_event = NewEvent {
        timeline_id,
        title,
        body,
        end_time,
    };

    diesel::insert_into(tasks::table)
        .values(&new_event)
        .get_result(conn)
        .expect("Error saving new event")
}

pub fn create_task<'a>(
    conn: &PgConnection,
    timeline_id: i32,
    title: &'a str,
    body: Option<&'a str>,
    end_time: NaiveDateTime,
) -> Task {
    let new_task = NewTask {
        timeline_id,
        title,
        body,
        end_time,
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(conn)
        .expect("Error saving new task")
}
use chrono::naive::NaiveDateTime;
use super::schema::events;
use super::diesel::prelude::*;
use super::diesel::pg::PgConnection;

#[derive(Queryable)]
#[diesel(belongs_to(Timeline))]
pub struct Event {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub done: Option<bool>,
}

#[derive(Insertable)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub timeline_id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="events"]
pub struct NewTask<'a> {
    pub timeline_id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub done: Option<bool>,
}

pub fn create_event<'a>(conn: &PgConnection, timeline_id: i32, title: &'a str, body: Option<&'a str>) -> Event {

    let new_event = NewEvent {
        timeline_id,
        title,
        body,
    };

    diesel::insert_into(events::table)
        .values(&new_event)
        .get_result(conn)
        .expect("Error saving new event")
}

pub fn create_task<'a>(conn: &PgConnection, timeline_id: i32, title: &'a str, body: Option<&'a str>) -> Event {

    let new_task = NewTask {
        timeline_id,
        title,
        body,
        done: Some(false),
    };

    diesel::insert_into(events::table)
        .values(&new_task)
        .get_result(conn)
        .expect("Error saving new event")
}
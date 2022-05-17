use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::events;
use crate::table::timelines::Timeline;
use chrono::naive::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Timeline)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub timeline_id: i32,
    pub title: &'a str,
    pub body: Option<&'a str>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "events"]
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
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Event {
    let new_event = NewEvent {
        timeline_id,
        title,
        body,
        start_time,
        end_time,
    };

    diesel::insert_into(events::table)
        .values(&new_event)
        .get_result(conn)
        .expect("Error saving new event")
}

use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::sub_events;
use crate::table::events::Event;
use chrono::naive::NaiveDateTime;


#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Event)]
#[table_name = "sub_events"]
pub struct SubEvent {
    pub id: i32,
    pub event_id: i32,
    pub title: String,
    pub done: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "sub_events"]
pub struct NewSubEvent<'a> {
    pub event_id: i32,
    pub title: &'a str,
}

#[derive(Insertable)]
#[table_name = "sub_events"]
pub struct NewSubTask<'a> {
    pub event_id: i32,
    pub title: &'a str,
    pub done: Option<bool>,
}

pub fn create_event<'a>(
    conn: &PgConnection,
    event_id: i32,
    title: &'a str,
) -> SubEvent {
    let new_sub_event = NewSubEvent {
        event_id,
        title,
    };

    diesel::insert_into(sub_events::table)
        .values(&new_sub_event)
        .get_result(conn)
        .expect("Error saving new sub event")
}

pub fn create_task<'a>(
    conn: &PgConnection,
    event_id: i32,
    title: &'a str,
) -> SubEvent {
    let new_sub_task = NewSubTask {
        event_id,
        title,
        done: Some(false),
    };

    diesel::insert_into(sub_events::table)
        .values(&new_sub_task)
        .get_result(conn)
        .expect("Error saving new task")
}
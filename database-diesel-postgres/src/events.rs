use chrono::naive::NaiveDateTime;
use super::schema::timelines;
use super::diesel::prelude::*;
use super::diesel::pg::PgConnection;

#[derive(Queryable)]
#[diesel(belongs_to(Timeline))]
pub struct Event {
    id: i32,
    timeline_id: i32,
    title: String,
    body: Nullable<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    done: Nullable<bool>,
}
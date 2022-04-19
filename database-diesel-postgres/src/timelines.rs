use super::diesel::pg::PgConnection;
use super::diesel::prelude::*;
use super::schema::timelines;
use chrono::naive::NaiveDateTime;

#[derive(Queryable)]
pub struct Timeline {
    pub id: i32,
    pub title: String,
    pub public: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "timelines"]
pub struct NewTimeline<'a> {
    pub title: &'a str,
}

pub fn create_timeline<'a>(conn: &PgConnection, title: &'a str) -> Timeline {
    let new_timeline = NewTimeline { title: title };

    diesel::insert_into(timelines::table)
        .values(&new_timeline)
        .get_result(conn)
        .expect("Error saving new timeline")
}

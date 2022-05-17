use juniper::GraphQLObject;
use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::timelines;
use chrono::naive::NaiveDateTime;

#[derive(GraphQLObject, Clone, Queryable, Identifiable)]
#[table_name = "timelines"]
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

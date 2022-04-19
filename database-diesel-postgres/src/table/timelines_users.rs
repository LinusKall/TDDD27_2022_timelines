use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::models::enums::*;
use crate::schema::timelines_users;
use chrono::naive::NaiveDateTime;

#[derive(Debug, Queryable)]
#[diesel(belongs_to(Timeline))]
#[diesel(belongs_to(User))]
pub struct TimelinesUsers {
    pub timeline_id: i32,
    pub user_id: i32,
    pub relation: Clearance,
    pub color: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "timelines_users"]
pub struct NewTimelineUser<'a> {
    pub timeline_id: i32,
    pub user_id: i32,
    pub relation: &'a Clearance,
    pub color: &'a str,
}

pub fn add_user_to_timeline<'a>(
    conn: &PgConnection,
    timeline_id: i32,
    user_id: i32,
    relation: &'a Clearance,
    color: &'a str,
) -> TimelinesUsers {
    let new_timeline_user = NewTimelineUser {
        timeline_id,
        user_id,
        relation,
        color,
    };

    diesel::insert_into(timelines_users::table)
        .values(&new_timeline_user)
        .get_result(conn)
        .expect("Error saving new event")
}

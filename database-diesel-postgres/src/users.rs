/*use chrono::naive::NaiveDateTime;
use super::schema::users;
use super::diesel::prelude::*;
use super::diesel::pg::PgConnection;*/

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Queryable)]
pub struct UserWithPassword {
    pub user: User,
    pub password: String,
}

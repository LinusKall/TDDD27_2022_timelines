/*use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::users;*/
use chrono::naive::NaiveDateTime;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct UserWithPassword {
    pub user: User,
    pub hashed_password: String,
}

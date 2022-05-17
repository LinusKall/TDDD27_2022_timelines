use crate::diesel::pg::PgConnection;
use crate::diesel::prelude::*;
use crate::schema::users;
use chrono::naive::NaiveDateTime;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub hashed_password: &'a str,
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    email: &'a str,
    password: &'a str,
) -> User {
    let new_user = NewUser {
        username,
        email,
        hashed_password: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

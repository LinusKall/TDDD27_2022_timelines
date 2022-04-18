extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use db::schema::timelines::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(timelines.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting timeline");

    println!("Deleted {} posts", num_deleted);
}

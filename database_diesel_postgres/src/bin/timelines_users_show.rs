extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::*;
use self::diesel::prelude::*;
use self::table::timelines_users::*;

fn main() {
    use db::schema::timelines_users::dsl::*;

    let connection = establish_connection();
    let results = timelines_users
        .load::<TimelinesUsers>(&connection)
        .expect("Error loading users");

    println!("Displaying {} timeline-user connections", results.len());
    for timeline_user in results {
        println!("{:?}", timeline_user);
    }
}

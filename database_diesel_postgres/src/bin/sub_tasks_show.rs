extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::*;
use self::diesel::prelude::*;
use self::table::sub_events::*;

fn main() {
    use db::schema::sub_events::dsl::*;

    let connection = establish_connection();
    let results = sub_events
        .load::<SubEvent>(&connection)
        .expect("Error loading users");

    println!("Displaying {} sub events", results.len());
    for sub_event in results {
        println!("{:?}", sub_event);
    }
}
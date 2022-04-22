extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::establish_connection;
use self::db::table::events::*;
use self::diesel::prelude::*;

fn main() {
    use db::schema::events::dsl::*;

    let connection = establish_connection();
    let results = events
        .load::<Event>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} tasks", results.len());
    for event in results {
        match event.done {
            Some(_) => println!("{:?}", event),
            None => {},
        }
    }
}

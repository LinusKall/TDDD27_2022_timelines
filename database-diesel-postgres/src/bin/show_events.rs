extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::establish_connection;
use self::db::events::*;
use self::diesel::prelude::*;
use std::env::args;


fn main() {
    use db::schema::events::dsl::*;

    let tl_id: i32 = args().nth(1).expect("Expected a timeline ID as an argument.").parse().unwrap();

    let connection = establish_connection();
    let results = events
        .filter(timeline_id.eq(tl_id))
        .limit(5)
        .load::<Event>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} events", results.len());
    for event in results {
        match event.body {
            Some(b) => println!("\n{}: {}\n{}", event.created_at, event.title, b),
            None => println!("\n{}: {}", event.created_at, event.title),
        }
    }
}
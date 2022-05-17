extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::establish_connection;
use self::db::table::timelines::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like the title of the timeline to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim(); // Drop the newline character

    let timeline = create_timeline(&connection, title);

    println!("\nSaved timeline {:?} with id {:?}", title, timeline.id);
}

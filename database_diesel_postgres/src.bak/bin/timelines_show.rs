extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::*;
use self::diesel::prelude::*;
use self::table::timelines::*;

fn main() {
    use db::schema::timelines::dsl::*;

    let connection = establish_connection();
    let results = timelines
        .load::<Timeline>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} timelines", results.len());
    for timeline in results {
        println!("{:?}", timeline);
    }
}

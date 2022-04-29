extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::establish_connection;
use self::db::table::tasks::*;
use self::diesel::prelude::*;

fn main() {
    use db::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks
        .load::<Task>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} tasks", results.len());
    for event in results {
        println!("{:?}", event);
    }
}

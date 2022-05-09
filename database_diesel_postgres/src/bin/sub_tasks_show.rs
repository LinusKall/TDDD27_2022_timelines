extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::*;
use self::diesel::prelude::*;
use self::table::sub_tasks::*;

fn main() {
    use db::schema::sub_tasks::dsl::*;

    let connection = establish_connection();
    let results = sub_tasks
        .load::<SubTask>(&connection)
        .expect("Error loading users");

    println!("Displaying {} sub events", results.len());
    for sub_task in results {
        println!("{:?}", sub_task);
    }
}

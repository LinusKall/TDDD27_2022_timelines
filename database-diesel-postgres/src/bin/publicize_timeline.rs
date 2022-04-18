extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::establish_connection;
use self::db::timelines::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use db::schema::timelines::dsl::{public, timelines};

    let id = args()
        .nth(1)
        .expect("`publicize_timeline` requires a timeline ID")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let timeline = diesel::update(timelines.find(id))
        .set(public.eq(true))
        .get_result::<Timeline>(&connection)
        .expect(&format!("Unable to find timeline {}", id));
    println!("Publicized timeline {}", timeline.title);
}

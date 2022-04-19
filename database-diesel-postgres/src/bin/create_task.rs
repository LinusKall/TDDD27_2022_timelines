extern crate database_diesel_postgres as db;
extern crate diesel;

use chrono::NaiveDateTime;

use self::db::establish_connection;
use self::db::events::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the id of the timeline you wish to add a task to?");
    let mut id = String::new();
    stdin().read_line(&mut id).unwrap();
    let id = id.trim().parse().unwrap();

    println!("\nWhat would you like the title of the task to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim(); // Drop the newline character

    println!(
        "\nOk! Let's write a description for {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();
    let body = if body.trim().is_empty() {
        None
    } else {
        Some(body.as_str())
    };

    println!("Type the end time with the format: YYYY-mm-dd HH:MM");
    let mut end_time = String::new();
    stdin().read_to_string(&mut end_time).unwrap();
    let end_time = NaiveDateTime::parse_from_str(end_time.trim(), "%Y-%m-%d %H:%M").expect("Could not parse the end time");

    let event = create_task(&connection, id, title, body, end_time);

    println!("\nSaved event {:?} with id {:?}", title, event.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

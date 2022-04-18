extern crate database_diesel_postgres as db;
extern crate diesel;

use self::db::establish_connection;
use self::db::events::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the id of the timeline you wish to add an event to?");
    let mut id = String::new();
    stdin().read_line(&mut id).unwrap();
    let id = id.trim().parse().unwrap();

    println!("\nWhat would you like the title of the event to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim(); // Drop the newline character
    
    println!("\nOk! Let's write a description for {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let body = if body.trim().is_empty() {
        None
    } else {
        Some(body.as_str())
    };

    let event = create_event(&connection, id, title, body);

    println!("\nSaved event {:?} with id {:?}", title, event.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
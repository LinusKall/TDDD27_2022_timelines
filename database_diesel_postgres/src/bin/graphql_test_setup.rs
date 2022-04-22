extern crate database_diesel_postgres as db;
extern crate diesel;

use chrono::NaiveDate;
use diesel::prelude::*;
use self::db::*;

fn main() {
    let connection = establish_connection();

    delete_sub_events(&connection);
    delete_events(&connection);
    delete_timelines_users(&connection);
    delete_timelines(&connection);
    delete_users(&connection);

    let user = table::users::create_user(&connection, "username", "username@mail.com", "password");
    let timeline = table::timelines::create_timeline(&connection, "timeline 1");
    let timeline_user = table::timelines_users::add_user_to_timeline(
        &connection, 
        timeline.id, 
        user.id, 
        &models::enums::Clearance::Owner,
        "#1a7ca9"
    );
    let event = table::events::create_event(
        &connection, 
        timeline.id, 
        "party!", 
        Some("västerås!"), 
        Some(NaiveDate::from_ymd(2022, 4, 30).and_hms(16, 0, 0)),
        NaiveDate::from_ymd(2022, 4, 30).and_hms(23, 0, 0)
    );
    let task = table::events::create_event(
        &connection, 
        timeline.id, 
        "prepare!", 
        Some("västerås!"), 
        None,
        NaiveDate::from_ymd(2022, 4, 28).and_hms(20, 0, 0)
    );
    let sub_task = table::sub_events::create_sub_task(
        &connection, 
        task.id, 
        "pack clothes!",
    );
    
    println!("Created user: {:#?}", user);
    println!("Created timeline: {:#?}", timeline);
    println!("Created timeline-user connection: {:#?}", timeline_user);
    println!("Created event: {:#?}", event);
    println!("Created task: {:#?}", task);
    println!("Created sub_task: {:#?}", sub_task);



}


fn delete_sub_events(connection: &PgConnection) {
    use crate::db::schema::sub_events::dsl::*;
    let num_deleted = diesel::delete(sub_events)
        .execute(connection)
        .expect("Error deleting sub_events");
    println!("Deleted {} sub-events.", num_deleted);
}

fn delete_events(connection: &PgConnection) {
    use crate::db::schema::events::dsl::*;
    let num_deleted = diesel::delete(events)
        .execute(connection)
        .expect("Error deleting sub_events");
    println!("Deleted {} events.", num_deleted);
}

fn delete_timelines_users(connection: &PgConnection) {
    use crate::db::schema::timelines_users::dsl::*;
    let num_deleted = diesel::delete(timelines_users)
        .execute(connection)
        .expect("Error deleting sub_events");
    println!("Deleted {} timeline-user connections.", num_deleted);
}

fn delete_timelines(connection: &PgConnection) {
    use crate::db::schema::timelines::dsl::*;
    let num_deleted = diesel::delete(timelines)
        .execute(connection)
        .expect("Error deleting sub_events");
    println!("Deleted {} timelines.", num_deleted);
}

fn delete_users(connection: &PgConnection) {
    use crate::db::schema::users::dsl::*;
    let num_deleted = diesel::delete(users)
        .execute(connection)
        .expect("Error deleting sub_events");
    println!("Deleted {} users.", num_deleted);
}
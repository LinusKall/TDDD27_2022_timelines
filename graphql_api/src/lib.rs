use chrono::naive::NaiveDateTime;
use juniper::GraphQLObject;
// use graphql_client::{GraphQLQuery, Response};
use serde::Deserialize;

#[derive(GraphQLObject, Deserialize)]
pub struct Userdata {
    pub id: i32,
    pub timelines: Vec<Timeline>,
}

#[derive(GraphQLObject, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLObject, Deserialize)]
pub struct Timeline {
    pub id: i32,
    pub color: String,
    pub events: Vec<Event>,
    pub tasks: Vec<Task>,
}

#[derive(GraphQLObject, Deserialize)]
pub struct Event {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(GraphQLObject, Deserialize)]
pub struct Task {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub done: bool,
    pub end_time: NaiveDateTime,
    pub sub_tasks: Vec<SubTask>,
}

#[derive(GraphQLObject, Deserialize)]
pub struct SubTask {
    pub id: i32,
    pub task_id: i32,
    pub title: String,
    pub done: bool,
}

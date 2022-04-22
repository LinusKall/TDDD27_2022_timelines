use chrono::naive::NaiveDateTime;
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Userdata {
    pub id: i32,
    pub timelines: Vec<Timeline>,   
}

#[derive(GraphQLObject)]
pub struct Timeline {
    pub id: i32,
    pub color: String,
    pub events: Vec<Event>,
    pub tasks: Vec<Task>,
}

#[derive(GraphQLObject)]
pub struct Event {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(GraphQLObject)]
pub struct Task {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub done: bool,
    pub end_time: NaiveDateTime,
    pub sub_tasks: Vec<SubTask>,
}

#[derive(GraphQLObject)]
pub struct SubTask {
    pub id: i32,
    pub task_id: i32,
    pub title: String,
    pub done: bool,
}

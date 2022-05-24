use entity::async_graphql::{self, SimpleObject};

use crate::graphql::schema::DateTimeWrapper;

#[derive(SimpleObject, Default, PartialEq, Clone)]
pub struct UserData {
    pub id: i32,
    pub timelines: Vec<NestedTimeline>,
}

// #[derive(SimpleObject, Default, PartialEq, Clone)]
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub email: String,
//     pub hashed_password: String,
//     pub created_at: DateTime,
//     pub updated_at: DateTime,
// }

#[derive(SimpleObject, Default, PartialEq, Clone)]
pub struct NestedTimeline {
    pub id: i32,
    pub title: String,
    pub color: String,
    pub events: Vec<NestedEvent>,
    pub tasks: Vec<NestedTask>,
}

#[derive(SimpleObject, Default, PartialEq, Clone)]
pub struct NestedEvent {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub start_time: DateTimeWrapper,
    pub end_time: DateTimeWrapper,
}

#[derive(SimpleObject, Default, PartialEq, Clone)]
pub struct NestedTask {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub done: bool,
    pub end_time: DateTimeWrapper,
    pub sub_tasks: Vec<NestedSubTask>,
}

#[derive(SimpleObject, Default, PartialEq, Clone)]
pub struct NestedSubTask {
    pub id: i32,
    pub task_id: i32,
    pub title: String,
    pub done: bool,
}

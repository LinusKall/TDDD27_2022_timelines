use entity::async_graphql::{self, SimpleObject};
use cynic::{impl_scalar, QueryFragment};

use crate::graphql::schema::DateTimeWrapper;

mod schema {
    cynic::use_schema!("../frontend_yew/graphql/schema.graphql");
}

type DateTimeUtc = chrono::DateTime<chrono::Utc>;
impl_scalar!(DateTimeUtc, schema::DateTime);

#[derive(Default, PartialEq, Clone, QueryFragment)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
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

#[derive(Default, PartialEq, Clone, QueryFragment)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
pub struct NestedTimeline {
    pub id: i32,
    pub title: String,
    pub color: String,
    pub events: Vec<NestedEvent>,
    pub tasks: Vec<NestedTask>,
}

#[derive(Default, PartialEq, Clone, QueryFragment)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
pub struct NestedEvent {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub start_time: DateTimeWrapper,
    pub end_time: DateTimeWrapper,
}

#[derive(Default, PartialEq, Clone, QueryFragment)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
pub struct NestedTask {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub done: bool,
    pub end_time: DateTimeWrapper,
    pub sub_tasks: Vec<NestedSubTask>,
}

#[derive(Default, PartialEq, Clone, QueryFragment)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
pub struct NestedSubTask {
    pub id: i32,
    pub task_id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Model")]
pub struct DateTimeWrapper {
    pub inner: DateTimeUtc,
}
impl Default for DateTimeWrapper {
    fn default() -> Self {
        Self {
            inner: chrono::Utc::now(),
        }
    }
}
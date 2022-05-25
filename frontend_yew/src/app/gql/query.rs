use cynic::{impl_scalar, QueryFragment};


type DateTimeUtc = chrono::DateTime<chrono::Utc>;
impl_scalar!(DateTimeUtc, schema::DateTime);

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

#[derive(cynic::FragmentArguments)]
pub struct GetUserTimelinesArguments {
    pub user_id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserTimelinesArguments"
)]
pub struct GetUserTimelinesById {
    #[arguments(user_id = &args.user_id)]
    pub get_user_timelines_by_id: Vec<UserTimeline>,
}

#[derive(cynic::FragmentArguments)]
pub struct CreateUserTimelineArguments {
    pub user_id: i32,
    pub title: String,
    pub public: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "CreateUserTimelineArguments"
)]
pub struct CreateUserTimeline {
    #[arguments(input = &args)]
    pub create_user_timeline: UserTimeline,
}

#[derive(cynic::FragmentArguments)]
pub struct GetTasksByIdArguments {
    pub timeline_id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "GetTaskByIdArguments"
)]
pub struct GetTaskById {
    #[arguments(timeline_id = &args.timeline_id)]
    pub get_tasks_by_id: Vec<Task>,
}

// #[derive(cynic::FragmentArguments)]
// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(
//     schema_path = "graphql/schema.graphql",
//     graphql_type = "Query",
//     argument_struct = "GetUserTimelinesArguments"
// )]

#[derive(Debug, QueryFragment, Clone, PartialEq, Default)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "UserTimeline",
)]
pub struct UserTimeline {
    pub props_id: i32,
    pub user_id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub color: String,
    pub props_created_at: DateTimeUtc,
    pub props_updated_at: DateTimeUtc,
    pub timeline_created_at: DateTimeUtc,
    pub timeline_updated_at: DateTimeUtc,
}

#[derive(Debug, QueryFragment, Clone, PartialEq, Default)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Task",
)]
pub struct Task {
    pub id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub done: bool,
    pub end_time: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

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

#[derive(QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserTimelinesArguments"
)]
pub struct GetUserTimelinesById {
    #[arguments(user_id = &args.user_id)]
    pub get_user_timelines_by_id: Vec<UserTimeline>,
}

#[derive(cynic::InputObject, cynic::FragmentArguments)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "CreateUserTimelineInput"
)]
pub struct CreateUserTimelineArguments {
    pub user_id: i32,
    pub title: String,
    pub public: bool,
}

#[derive(QueryFragment, Debug)]
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

#[derive(QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetTasksByIdArguments"
)]
pub struct GetTasksById {
    #[arguments(timeline_id = &args.timeline_id)]
    pub get_tasks_by_id: Vec<Task>,
}

#[derive(Debug, QueryFragment, Clone, PartialEq)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "UserTimeline")]
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

#[derive(Debug, QueryFragment, Clone, PartialEq)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "Task")]
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

impl UserTimeline {
    pub fn default() -> Self {
        Self {
            props_id: 0,
            user_id: 0,
            timeline_id: 0,
            title: "".to_owned(),
            color: "".to_owned(),
            props_created_at: chrono::offset::Utc::now(),
            props_updated_at: chrono::offset::Utc::now(),
            timeline_created_at: chrono::offset::Utc::now(),
            timeline_updated_at: chrono::offset::Utc::now(),
        }
    }
}

impl Task {
    pub fn default() -> Self {
        Self {
            id: 0,
            timeline_id: 0,
            title: "".to_owned(),
            body: None,
            done: false,
            end_time: chrono::offset::Utc::now(),
            created_at: chrono::offset::Utc::now(),
            updated_at: chrono::offset::Utc::now(),
        }
    }
}
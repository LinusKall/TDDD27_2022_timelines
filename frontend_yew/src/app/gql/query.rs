use cynic::{impl_scalar, QueryFragment};

type DateTimeUtc = chrono::DateTime<chrono::Utc>;
impl_scalar!(DateTimeUtc, schema::DateTime);

mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

#[derive(Debug, cynic::Enum, Clone, Copy, PartialEq)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "ClearanceMapping"
)]
pub enum ClearanceMapping {
    Moderator,
    Owner,
    Subscriber,
}

// GetUserTimeline
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

// CreateUserTimeline
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

// CreateTask
#[derive(cynic::InputObject, cynic::FragmentArguments)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "CreateTaskInput"
)]
pub struct CreateTaskArguments {
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub end_time: Option<DateTimeUtc>,
}

#[derive(QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "CreateTaskArguments"
)]
pub struct CreateTask {
    #[arguments(input = &args)]
    pub create_task: Task,
}

// GetTasks
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
    pub relation: ClearanceMapping,
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
    pub end_time: Option<DateTimeUtc>,
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
            relation: ClearanceMapping::Owner,
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
            end_time: None,
            created_at: chrono::offset::Utc::now(),
            updated_at: chrono::offset::Utc::now(),
        }
    }
}

// Login component
// ---------------------------------
#[derive(cynic::FragmentArguments)]
pub struct GetUserIdArguments {
    pub username: String,
    pub password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserIdArguments"
)]
pub struct GetUserId {
    #[arguments(username = &args.username, password = &args.password)]
    pub get_user_id: Option<i32>,
}
// ---------------------------------

// Signup component
// ---------------------------------
#[derive(cynic::FragmentArguments, cynic::InputObject)]
#[cynic(schema_path = "graphql/schema.graphql")]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "CreateUserInput"
)]
pub struct CreateUser {
    #[arguments(input = &args)]
    pub create_user: User,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "User")]
pub struct User {
    pub id: i32,
}
// ---------------------------------

// Account_info component
// ---------------------------------
#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "graphql/schema.graphql", graphql_type = "UserInfo")]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(cynic::FragmentArguments)]
pub struct GetUserInfoArgs {
    pub user_id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Query",
    argument_struct = "GetUserInfoArgs"
)]
pub struct GetUserInfo {
    #[arguments(user_id = &args.user_id)]
    pub get_user_info: UserInfo,
}

#[derive(cynic::FragmentArguments)]
pub struct DeleteUserInput {
    pub user_id: i32,
    pub password: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "DeleteUserInput"
)]
pub struct DeleteUser {
    #[arguments(user_id = &args.user_id, password = &args.password)]
    pub delete_user: DeleteUserResult,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "DeleteUserResult"
)]
pub struct DeleteUserResult {
    pub success: bool,
    pub rows_affected: i32,
}
// ---------------------------------

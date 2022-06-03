use super::query::schema;
use super::query::UserTimeline;
use cynic::QueryFragment;

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

#[derive(cynic::FragmentArguments, cynic::InputObject)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "UpdateUserTimelineInput"
)]
pub struct UpdateUserTimelineInput {
    pub props_id: i32,
    pub timeline_id: i32,
    pub title: Option<String>,
    pub color: Option<String>,
    pub relation: Option<ClearanceMapping>,
}

#[derive(Debug, QueryFragment, Clone, PartialEq)]
#[cynic(
    schema_path = "graphql/schema.graphql",
    graphql_type = "Mutation",
    argument_struct = "UpdateUserTimelineInput"
)]
pub struct UpdateUserTimeline {
    #[arguments(input = &args)]
    pub update_user_timeline: UserTimeline,
}

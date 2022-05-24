mod schema {
    cynic::use_schema!("graphql/schema.graphql");
}

// #[derive(cynic::FragmentArguments)]
// #[derive(cynic::QueryFragment, Debug)]
// #[cynic(
//     schema_path = "graphql/schema.graphql",
//     graphql_type = "Query",
//     argument_struct = "GetUserTimelinesArguments"
// )]

#[derive(Debug)]
pub struct UserTimeline {
    props_id: i32,
    user_id: i32,
    timeline_id: i32,
    title: String,
    user_timeline_relation: i32,
    color: String,
    props_created_at: DateTimeUtc,
    props_updated_at: DateTimeUtc,
    timeline_created_at: DateTimeUtc,
    timeline_updated_at: DateTimeUtc,
}

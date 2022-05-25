use async_graphql::SimpleObject;
use entity::{async_graphql, sea_orm_active_enums::ClearanceMapping};
use sea_orm::entity::prelude::*;
use sea_orm::FromQueryResult;

#[derive(Debug, FromQueryResult, SimpleObject)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[allow(unused)]
#[derive(Debug, FromQueryResult, SimpleObject)]
pub struct UserTimeline {
    props_id: i32,
    user_id: i32,
    timeline_id: i32,
    title: String,
    relation: ClearanceMapping,
    user_timeline_relation: i32,
    color: String,
    props_created_at: DateTimeUtc,
    props_updated_at: DateTimeUtc,
    timeline_created_at: DateTimeUtc,
    timeline_updated_at: DateTimeUtc,
}

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
    pub props_id: i32,
    pub user_id: i32,
    pub timeline_id: i32,
    pub title: String,
    pub relation: ClearanceMapping,
    pub color: String,
    pub props_created_at: DateTimeUtc,
    pub props_updated_at: DateTimeUtc,
    pub timeline_created_at: DateTimeUtc,
    pub timeline_updated_at: DateTimeUtc,
}

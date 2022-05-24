use async_graphql::{Context, Object, Result, SimpleObject};
use entity::{async_graphql, timelines, timelines_users};
use sea_orm::entity::prelude::*;
use sea_orm::{query::*, EntityTrait, FromQueryResult};

use crate::db::Database;

#[allow(unused)]
#[derive(Debug, FromQueryResult, SimpleObject)]
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

#[derive(Default)]
pub struct TimelinesQuery;

#[Object]
impl TimelinesQuery {
    async fn get_timelines_by_id(
        &self,
        ctx: &Context<'_>,
        user_id: i32,
    ) -> Result<Vec<UserTimeline>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(timelines_users::Entity::find()
            .having(timelines_users::Column::UserId.eq(user_id))
            .inner_join(timelines::Entity)
            .select_only()
            .column_as(timelines_users::Column::Id, "props_id")
            .column_as(timelines_users::Column::UserId, "user_id")
            .column_as(timelines_users::Column::TimelineId, "timeline_id")
            .column_as(timelines::Column::Title, "title")
            .column_as(timelines_users::Column::Relation, "user_timeline_relation")
            .column_as(timelines_users::Column::Color, "color")
            .column_as(timelines_users::Column::CreatedAt, "props_created_at")
            .column_as(timelines_users::Column::UpdatedAt, "props_updated_at")
            .column_as(timelines::Column::CreatedAt, "timeline_created_at")
            .column_as(timelines::Column::UpdatedAt, "timeline_updated_at")
            .group_by(timelines::Column::Id)
            .group_by(timelines_users::Column::Id)
            .into_model::<UserTimeline>()
            .all(db.get_connection())
            .await?)
    }

    async fn get_timelines(&self, ctx: &Context<'_>) -> Result<Vec<timelines::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(timelines::Entity::find()
            .filter(timelines::Column::Public.eq(true))
            .all(db.get_connection())
            .await?)
    }

    async fn get_timeline_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<timelines::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(timelines::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }
}

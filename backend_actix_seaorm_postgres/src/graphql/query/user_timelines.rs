use async_graphql::{Context, Object, Result};
use entity::{async_graphql, timelines, timelines_users};
use sea_orm::entity::prelude::*;
use sea_orm::{query::*, EntityTrait};

use crate::db::Database;
use crate::graphql::custom_types::UserTimeline;

#[derive(Default)]
pub struct UserTimelinesQuery;

#[Object]
impl UserTimelinesQuery {
    async fn get_user_timelines_by_id(
        &self,
        ctx: &Context<'_>,
        user_id: i32,
    ) -> Result<Vec<UserTimeline>> {
        let db = ctx.data::<Database>().unwrap();

        println!("Finding user timelines");

        let res = timelines_users::Entity::find()
            .having(timelines_users::Column::UserId.eq(user_id))
            .inner_join(timelines::Entity)
            .select_only()
            .column_as(timelines_users::Column::Id, "props_id")
            .column_as(timelines_users::Column::UserId, "user_id")
            .column_as(timelines_users::Column::TimelineId, "timeline_id")
            .column_as(timelines::Column::Title, "title")
            .column_as(timelines_users::Column::Relation, "relation")
            .column_as(timelines_users::Column::Color, "color")
            .column_as(timelines_users::Column::CreatedAt, "props_created_at")
            .column_as(timelines_users::Column::UpdatedAt, "props_updated_at")
            .column_as(timelines::Column::CreatedAt, "timeline_created_at")
            .column_as(timelines::Column::UpdatedAt, "timeline_updated_at")
            .group_by(timelines::Column::Id)
            .group_by(timelines_users::Column::Id)
            .into_model::<UserTimeline>()
            .all(db.get_connection())
            .await?;

        println!("Got res: {:#?}", res);

        Ok(res)
    }
}

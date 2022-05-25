use async_graphql::{Context, Object, Result};
use entity::{async_graphql, timelines};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::db::Database;

#[derive(Default)]
pub struct TimelinesQuery;

#[Object]
impl TimelinesQuery {
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

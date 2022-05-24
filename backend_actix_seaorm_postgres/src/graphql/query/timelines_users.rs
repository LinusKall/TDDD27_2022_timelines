use async_graphql::{Context, Object, Result};
use entity::{async_graphql, timelines_users};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::db::Database;

#[derive(Default)]
pub struct TimelinesUsersQuery;

#[Object]
impl TimelinesUsersQuery {
    async fn get_timelines_users(
        &self,
        ctx: &Context<'_>,
        user_id: i32,
    ) -> Result<Vec<timelines_users::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(timelines_users::Entity::find()
            .filter(timelines_users::Column::UserId.eq(user_id))
            .all(db.get_connection())
            .await?)
    }
}

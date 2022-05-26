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

        let mut join = timelines_users::Entity::find()
            .filter(timelines_users::Column::UserId.eq(user_id))
            .find_also_related(timelines::Entity)
            .all(db.get_connection())
            .await?;

        let mut res = Vec::new();

        for (tu, to) in join.drain(..) {
            let t = to.unwrap();
            res.push(UserTimeline {
                props_id: tu.id,
                user_id: tu.user_id,
                timeline_id: tu.timeline_id,
                title: t.title.to_owned(),
                relation: tu.relation,
                color: tu.color.to_owned(),
                props_created_at: tu.created_at,
                props_updated_at: tu.updated_at,
                timeline_created_at: t.created_at,
                timeline_updated_at: t.updated_at,
            });
        }

        Ok(res)
    }
}

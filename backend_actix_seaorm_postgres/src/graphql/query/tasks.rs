use std::cmp::Ordering;

use async_graphql::{Context, Object, Result};
use entity::{async_graphql, tasks};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::db::Database;

#[derive(Default)]
pub struct TasksQuery;

#[Object]
impl TasksQuery {
    async fn get_tasks_by_id(
        &self,
        ctx: &Context<'_>,
        timeline_id: i32,
    ) -> Result<Vec<tasks::Model>> {
        let db = ctx.data::<Database>().unwrap();

        let mut tasks = tasks::Entity::find()
            .filter(tasks::Column::TimelineId.eq(timeline_id))
            .all(db.get_connection())
            .await?;

        tasks.sort_unstable_by(|a, b| match a.done.cmp(&b.done) {
            Ordering::Equal => a.created_at.cmp(&b.created_at),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        });

        Ok(tasks)
    }
}

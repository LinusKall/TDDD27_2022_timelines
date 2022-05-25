use async_graphql::{Context, Object, Result};
use entity::{async_graphql, sub_tasks};
use sea_orm::entity::prelude::*;
use sea_orm::{query::*, EntityTrait};

use crate::db::Database;

#[derive(Default)]
pub struct SubTasksQuery;

#[Object]
impl SubTasksQuery {
    async fn get_sub_tasks_by_id(
        &self,
        ctx: &Context<'_>,
        task_id: i32,
    ) -> Result<Vec<sub_tasks::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(sub_tasks::Entity::find()
            .having(sub_tasks::Column::TaskId.eq(task_id))
            .all(db.get_connection())
            .await?)
    }
}

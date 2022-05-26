use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::tasks;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::db::Database;

#[derive(InputObject)]
pub struct CreateTaskInput {
    pub timeline_id: i32,
    pub title: String,
    pub body: Option<String>,
    pub end_time: Option<DateTimeUtc>,
}

#[derive(SimpleObject)]
pub struct DeleteTaskResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct TasksMutation;

#[Object]
impl TasksMutation {
    pub async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> Result<tasks::Model> {
        let db = ctx.data::<Database>().unwrap();

        let task = tasks::ActiveModel {
            timeline_id: Set(input.timeline_id),
            title: Set(input.title),
            body: Set(input.body),
            end_time: Set(input.end_time),
            ..Default::default()
        };

        Ok(task.insert(db.get_connection()).await?)
    }

    pub async fn delete_task(&self, ctx: &Context<'_>, task_id: i32) -> Result<DeleteTaskResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = tasks::Entity::delete_by_id(task_id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteTaskResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            Err(entity::async_graphql::Error {
                message: format!("{} tasks were deleted", res.rows_affected),
                source: None,
                extensions: None,
            })
        }
    }
}

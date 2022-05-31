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

#[derive(InputObject)]
pub struct UpdateTaskInput {
    pub title: Option<String>,
    pub body: Option<String>,
    pub done: Option<bool>,
    pub end_time: Option<DateTimeUtc>,
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

    pub async fn update_task(
        &self,
        ctx: &Context<'_>,
        task_id: i32,
        input: UpdateTaskInput,
    ) -> Result<tasks::Model> {
        let db = ctx.data::<Database>().unwrap();

        let task = tasks::Entity::find_by_id(task_id)
            .one(db.get_connection())
            .await?;

        let mut task: tasks::ActiveModel = task.unwrap().into();

        if let Some(title) = input.title {
            task.title = Set(title.to_owned());
        }
        if let Some(body) = input.body {
            task.body = Set(Some(body.to_owned()));
        }
        if let Some(done) = input.done {
            task.done = Set(done);
        }
        if let Some(end_time) = input.end_time {
            println!("{:?}", &end_time);
            task.end_time = Set(Some(end_time));
        }

        Ok(task.update(db.get_connection()).await?)
    }
}

use async_graphql::{Context, Object, Result};
use entity::{async_graphql, events, sub_tasks, tasks, timelines, timelines_users, users};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use sea_orm::{query::*, FromQueryResult};

use super::user_data::*;
use crate::db::Database;
use crate::graphql::schema::DateTimeWrapper;
use entity::sea_orm_active_enums::ClearanceMapping;

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<users::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(users::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<users::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(users::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_id(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<Option<i32>> {
        let db = ctx.data::<Database>().unwrap();

        if let Some(user) = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .filter(users::Column::HashedPassword.eq(password))
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?
        {
            Ok(Some(user.id))
        } else {
            Ok(None)
        }
    }

    #[rustfmt::skip]
    async fn get_user_data(
        &self,
        ctx: &Context<'_>,
        id: i32,
        hashed_password: String, // TODO: Use tokens instead.
    ) -> Result<UserData> {
        let db = ctx.data::<Database>().unwrap();

        let _user = users::Entity::find()
            .filter(users::Column::Id.eq(id))
            .filter(users::Column::HashedPassword.eq(hashed_password))
            .one(db.get_connection())
            .await?
            .expect("Wrong ID and/or password");

        let mut user_data = UserData {
            id,
            timelines: Vec::new(),
        };

        #[allow(unused)]
        #[derive(Debug, FromQueryResult)]
        struct PropsTimes {
            pub tu_id: i32,
            pub tu_user_id: i32,
            pub tu_relation: ClearanceMapping,
            pub tu_color: String,
            pub tu_created_at: DateTimeUtc,
            pub tu_updated_at: DateTimeUtc,
            pub t_id: i32,
            pub t_title: String,
            pub t_public: bool,
            pub t_created_at: DateTimeUtc,
            pub t_updated_at: DateTimeUtc,
        }

        let properties_timelines = timelines_users::Entity::find()
            .having(timelines_users::Column::UserId.eq(id))
            .inner_join(timelines::Entity)
            .select_only()
            .column_as(timelines_users::Column::Id, "tu_id")
            .column_as(timelines_users::Column::Relation, "tu_relation")
            .column_as(timelines_users::Column::Color, "tu_color")
            .column_as(timelines_users::Column::CreatedAt, "tu_created_at")
            .column_as(timelines_users::Column::UpdatedAt, "tu_updated_at")
            .column_as(timelines::Column::Id, "t_id")
            .column_as(timelines::Column::Title, "t_title")
            .column_as(timelines::Column::Public, "t_public")
            .column_as(timelines::Column::CreatedAt, "t_created_at")
            .column_as(timelines::Column::UpdatedAt, "t_updated_at")
            .group_by(timelines::Column::Id)
            .group_by(timelines_users::Column::Id)
            .into_model::<PropsTimes>()
            .all(db.get_connection())
            .await?;

        for pt in properties_timelines.iter() {
            let events = events::Entity::find()
                .having(events::Column::TimelineId.eq(pt.t_id))
                .all(db.get_connection())
                .await?;
            let tasks = tasks::Entity::find()
                .having(tasks::Column::TimelineId.eq(pt.t_id))
                .all(db.get_connection())
                .await?;

            let mut timeline = NestedTimeline {
                id: pt.t_id,
                title: pt.t_title.to_owned(),
                color: pt.tu_color.to_owned(),
                events: Vec::new(),
                tasks: Vec::new(),
            };

            for event in events.iter() {
                timeline.events.push(NestedEvent {
                    id: event.id,
                    timeline_id: timeline.id,
                    title: event.title.to_owned(),
                    body: event.body.to_owned(),
                    start_time: DateTimeWrapper { inner: event.start_time },
                    end_time: DateTimeWrapper { inner: event.end_time },
                });
            }

            for task in tasks.iter() {
                let sub_tasks = sub_tasks::Entity::find()
                    .having(sub_tasks::Column::TaskId.eq(task.id))
                    .all(db.get_connection())
                    .await?
                    .iter()
                    .map(|sub_task| NestedSubTask {
                        id: sub_task.id,
                        task_id: task.id,
                        title: sub_task.title.to_owned(),
                        done: sub_task.done,
                    })
                    .collect();

                timeline.tasks.push(NestedTask {
                    id: task.id,
                    timeline_id: timeline.id,
                    title: task.title.clone(),
                    body: task.body.clone(),
                    done: task.done,
                    sub_tasks,
                    end_time: DateTimeWrapper { inner: task.end_time },
                });
            }
            user_data.timelines.push(timeline);
        }

        Ok(user_data)
    }
}

use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::{sea_orm_active_enums::ClearanceMapping, timelines, timelines_users, users};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::db::Database;
use crate::graphql::custom_types::UserTimeline;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateUserTimelineInput {
    pub user_id: i32,
    pub title: String,
    pub public: bool,
}

#[derive(SimpleObject)]
pub struct DeleteUserTimelineResult {
    pub success: bool,
    pub props_rows_affected: u64,
    pub timeline_rows_affected: Option<u64>,
}

#[derive(Default)]
pub struct UserTimelinesMutation;

#[Object]
impl UserTimelinesMutation {
    pub async fn create_user_timeline(
        &self,
        ctx: &Context<'_>,
        input: CreateUserTimelineInput,
    ) -> Result<UserTimeline> {
        let db = ctx.data::<Database>().unwrap();

        let _user = users::Entity::find_by_id(input.user_id)
            .one(db.get_connection())
            .await?
            .expect("The user trying to create a timeline does not exist.");

        let timeline_insertion = timelines::ActiveModel {
            title: Set(input.title),
            public: Set(input.public),
            ..Default::default()
        };

        let timeline = timeline_insertion.insert(db.get_connection()).await?;

        let timeline_user_insertion = timelines_users::ActiveModel {
            timeline_id: Set(timeline.id),
            user_id: Set(input.user_id),
            relation: Set(ClearanceMapping::Owner),
            color: Set("#888888".to_owned()),
            ..Default::default()
        };

        let timeline_user = timeline_user_insertion.insert(db.get_connection()).await?;

        Ok(UserTimeline {
            props_id: timeline_user.id,
            user_id: timeline_user.user_id,
            timeline_id: timeline_user.timeline_id,
            title: timeline.title,
            relation: timeline_user.relation,
            color: "#888888".to_owned(),
            props_created_at: timeline_user.created_at,
            props_updated_at: timeline_user.updated_at,
            timeline_created_at: timeline.created_at,
            timeline_updated_at: timeline.updated_at,
        })
    }

    pub async fn delete_user_timeline(
        &self,
        ctx: &Context<'_>,
        props_id: i32,
    ) -> Result<DeleteUserTimelineResult> {
        let db = ctx.data::<Database>().unwrap();

        let props = timelines_users::Entity::find_by_id(props_id)
            .one(db.get_connection())
            .await?
            .unwrap();

        let props_res = timelines_users::Entity::delete_by_id(props_id)
            .exec(db.get_connection())
            .await?;

        let timeline_res = if props.relation == ClearanceMapping::Owner {
            Some(
                timelines::Entity::delete_by_id(props.timeline_id)
                    .exec(db.get_connection())
                    .await?,
            )
        } else {
            None
        };

        if let Some(timeline_res) = timeline_res {
            if props_res.rows_affected <= 1 && timeline_res.rows_affected <= 1 {
                Ok(DeleteUserTimelineResult {
                    success: true,
                    props_rows_affected: props_res.rows_affected,
                    timeline_rows_affected: Some(timeline_res.rows_affected),
                })
            } else {
                Err(entity::async_graphql::Error {
                    message: format!(
                        "{} props and {} timelines were deleted",
                        props_res.rows_affected, timeline_res.rows_affected
                    ),
                    source: None,
                    extensions: None,
                })
            }
        } else {
            if props_res.rows_affected <= 1 {
                Ok(DeleteUserTimelineResult {
                    success: true,
                    props_rows_affected: props_res.rows_affected,
                    timeline_rows_affected: None,
                })
            } else {
                Err(entity::async_graphql::Error {
                    message: format!("{} props were deleted", props_res.rows_affected),
                    source: None,
                    extensions: None,
                })
            }
        }
    }
}

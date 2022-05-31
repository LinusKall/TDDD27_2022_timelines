use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::{sea_orm_active_enums::ClearanceMapping, tasks, timelines, timelines_users, users};
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

#[derive(InputObject)]
pub struct UpdateUserTimelineInput {
    props_id: i32,
    timeline_id: i32,
    title: Option<String>,
    color: Option<String>,
    relation: Option<ClearanceMapping>,
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
            tasks::Entity::delete_many()
                .exec(db.get_connection())
                .await?;
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

    pub async fn update_user_timeline(
        &self,
        ctx: &Context<'_>,
        input: UpdateUserTimelineInput,
    ) -> Result<UserTimeline> {
        let db = ctx.data::<Database>().unwrap();

        let timeline = timelines::Entity::find_by_id(input.timeline_id)
            .one(db.get_connection())
            .await?
            .unwrap();
        let props = timelines_users::Entity::find_by_id(input.props_id)
            .one(db.get_connection())
            .await?
            .unwrap();

        let mut timeline_active: timelines::ActiveModel = timeline.clone().into();
        let mut props_active: timelines_users::ActiveModel = props.clone().into();

        let mut timeline_update = false;
        let mut props_update = false;

        if let Some(title) = input.title {
            timeline_active.title = Set(title.to_owned());
            timeline_update = true;
        }
        if let Some(color) = input.color {
            props_active.color = Set(color.to_owned());
            props_update = true
        }
        if let Some(relation) = input.relation {
            props_active.relation = Set(relation.to_owned());
            props_update = true
        }

        let timeline: timelines::Model = if timeline_update {
            timeline_active.updated_at = Set(chrono::offset::Utc::now());
            timeline_active.update(db.get_connection()).await?
        } else {
            timeline
        };
        let props: timelines_users::Model = if props_update {
            props_active.updated_at = Set(chrono::offset::Utc::now());
            props_active.update(db.get_connection()).await?
        } else {
            props
        };

        Ok(UserTimeline {
            props_id: props.id,
            user_id: props.user_id,
            timeline_id: props.timeline_id,
            title: timeline.title.to_owned(),
            relation: props.relation,
            color: props.color.to_owned(),
            props_created_at: props.created_at,
            props_updated_at: props.updated_at,
            timeline_created_at: timeline.created_at,
            timeline_updated_at: timeline.updated_at,
        })
    }
}

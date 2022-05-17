use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, SimpleObject};
use entity::timelines;
use sea_orm::{ActiveModelTrait, Set, EntityTrait};

use crate::db::Database;

// I normally separate the input types into separate files/modules, but this is just
// a quick example.

#[derive(InputObject)]
pub struct CreateTimelineInput {
    pub title: String,
    pub public: bool,
}

#[derive(SimpleObject)]
pub struct DeleteResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct TimelinesMutation;

#[Object]
impl TimelinesMutation {
    pub async fn create_timeline(
        &self,
        ctx: &Context<'_>,
        input: CreateTimelineInput,
    ) -> Result<timelines::Model> {
        let db = ctx.data::<Database>().unwrap();

        let timeline = timelines::ActiveModel {
            title: Set(input.title),
            public: Set(input.public),
            ..Default::default()
        };

        Ok(timeline.insert(db.get_connection()).await?)
    }

    pub async fn delete_timeline(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = timelines::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }
}
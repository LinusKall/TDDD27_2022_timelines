//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use async_graphql::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "events")]
#[graphql(name = "Event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub timeline_id: i32,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub body: Option<String>,
    pub start_time: DateTimeUtc,
    pub end_time: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::timelines::Entity",
        from = "Column::TimelineId",
        to = "super::timelines::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Timelines,
}

impl Related<super::timelines::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Timelines.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

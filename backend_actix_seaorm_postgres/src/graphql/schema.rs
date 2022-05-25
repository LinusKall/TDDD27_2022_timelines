use async_graphql::Schema;
use entity::async_graphql::{self, SimpleObject};
use migration::{Migrator, MigratorTrait};
use sea_orm::entity::prelude::*;

use crate::{
    db::Database,
    graphql::{mutation::Mutation, query::Query, subscription::Subscription},
};

pub type AppSchema = Schema<Query, Mutation, Subscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Migrator::up(db.get_connection(), None).await.unwrap();

    Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(db)
    .finish()
}

#[derive(SimpleObject, Clone, Debug, PartialEq)]
pub struct DateTimeWrapper {
    pub inner: DateTimeUtc,
}
impl Default for DateTimeWrapper {
    fn default() -> Self {
        Self {
            inner: chrono::Utc::now(),
        }
    }
}

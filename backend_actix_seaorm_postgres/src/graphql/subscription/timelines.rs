use async_graphql::{Context, Result, Subscription};
use entity::{async_graphql, timelines};
use sea_orm::EntityTrait;

use crate::db::Database;

#[derive(Default)]
pub struct TimelinesSubscription;

#[Subscription]
impl TimelinesQuery {}

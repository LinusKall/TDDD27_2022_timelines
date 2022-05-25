// use async_graphql::{Context, Enum, Object, Result, Schema, Subscription};
// use entity::{async_graphql, timelines};
// use futures_util::{lock::Mutex, Stream, StreamExt};
// use sea_orm::EntityTrait;
// use slab::Slab;

// use crate::db::Database;
// use crate::graphql::schema::DateTimeWrapper;

// #[derive(Enum, Clone, Copy, PartialEq, Eq)]
// enum MutationType {
//     Created,
//     Deleted,
//     Modified,
// }

// struct StreamTimeline {
//     mutation_type: MutationType,
//     id: i32,
//     title: Option<String>,
//     public: Option<bool>,
//     updated_at: DateTimeWrapper,
// }

// #[derive(Default)]
// pub struct TimelinesSubscription;

// #[Subscription]
// impl TimelinesSubscription {
//     async fn stream_timelines(&self) -> impl Stream<Item = StreamTimeline> {
//         todo!()
//     }
// }

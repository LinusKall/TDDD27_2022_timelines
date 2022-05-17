use entity::async_graphql;

pub mod timelines;

pub use timelines::TimelinesQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(TimelinesQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(TimelinesQuery);

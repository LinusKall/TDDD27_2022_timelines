use entity::async_graphql;

pub mod timelines;
pub mod users;

pub use timelines::TimelinesQuery;
pub use users::UsersQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(TimelinesQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UsersQuery, TimelinesQuery);

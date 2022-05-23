use entity::async_graphql;

pub mod timelines;
pub mod users;

pub use timelines::TimelinesSubscription;
pub use users::UsersSubscription;

// Add your other ones here to create a unified Query object
// e.x. Query(TimelinesQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Subscription(UsersSubscription, TimelinesSubscription);

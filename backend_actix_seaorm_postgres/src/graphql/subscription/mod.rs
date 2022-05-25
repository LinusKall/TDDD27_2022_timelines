use entity::async_graphql;

pub mod integers;
pub mod timelines;
pub mod users;

pub use integers::IntegersSubscription;
// pub use timelines::TimelinesSubscription;
// pub use users::UsersSubscription;

// Add your other ones here to create a unified Subscription object
// e.x. Query(TimelinesSubscription, OtherSubscription, OtherOtherSubscription)
#[derive(async_graphql::MergedSubscription, Default)]
pub struct Subscription(
    IntegersSubscription, /* , UsersSubscription, TimelinesSubscription*/
);

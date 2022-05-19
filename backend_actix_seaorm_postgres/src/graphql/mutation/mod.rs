use entity::async_graphql;

pub mod timelines;
pub mod users;

pub use timelines::TimelinesMutation;
pub use users::UsersMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(TimelinesMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UsersMutation, TimelinesMutation);

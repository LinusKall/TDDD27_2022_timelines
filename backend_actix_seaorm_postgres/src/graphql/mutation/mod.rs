use entity::async_graphql;

pub mod timelines;

pub use timelines::TimelinesMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(TimelinesMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(TimelinesMutation);

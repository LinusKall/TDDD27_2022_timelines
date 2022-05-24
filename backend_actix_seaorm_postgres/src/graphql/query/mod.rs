use entity::async_graphql;

pub mod sub_tasks;
pub mod tasks;
pub mod timelines;
pub mod timelines_users;
pub mod user_data;
pub mod users;

pub use sub_tasks::SubTasksQuery;
pub use tasks::TasksQuery;
pub use timelines::TimelinesQuery;
pub use users::UsersQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(TimelinesQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UsersQuery, TimelinesQuery, TasksQuery);

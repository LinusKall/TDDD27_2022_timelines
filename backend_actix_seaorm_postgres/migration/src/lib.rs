pub use sea_orm_migration::prelude::*;

mod m20220517_114450_create_timelines;
mod m20220517_123500_create_users;
mod m20220517_123600_create_events;
mod m20220517_124200_create_timelines_users;
mod m20220517_124500_create_tasks;
mod m20220517_124900_create_sub_tasks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220517_114450_create_timelines::Migration),
            Box::new(m20220517_123500_create_users::Migration),
            Box::new(m20220517_123600_create_events::Migration),
            Box::new(m20220517_124200_create_timelines_users::Migration),
            Box::new(m20220517_124500_create_tasks::Migration),
            Box::new(m20220517_124900_create_sub_tasks::Migration),
        ]
    }
}

use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220517_123600_create_events"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE events (
            id SERIAL PRIMARY KEY,
            timeline_id SERIAL NOT NULL REFERENCES timelines (id),
            title TEXT NOT NULL,
            body TEXT,
            start_time TIMESTAMP NOT NULL,
            end_time TIMESTAMP NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE events";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220517_124900_create_timeslines"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE sub_tasks (
            id SERIAL PRIMARY KEY,
            task_id SERIAL NOT NULL REFERENCES tasks (id),
            title TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE sub_tasks";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for statement in UP_STATEMENTS {
            manager
                .get_connection()
                .execute_unprepared(statement)
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for statement in DOWN_STATEMENTS {
            manager
                .get_connection()
                .execute_unprepared(statement)
                .await?;
        }

        Ok(())
    }
}

const UP_STATEMENTS: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS projects (id TEXT PRIMARY KEY, title TEXT NOT NULL)",
    "CREATE TABLE IF NOT EXISTS sections (id TEXT PRIMARY KEY, title TEXT NOT NULL, project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE)",
    "CREATE TABLE IF NOT EXISTS cards (id TEXT PRIMARY KEY, title TEXT NOT NULL, description TEXT NOT NULL DEFAULT '', deadline TEXT, completed INTEGER NOT NULL DEFAULT 0, project_id TEXT REFERENCES projects(id) ON DELETE SET NULL, section_id TEXT REFERENCES sections(id) ON DELETE SET NULL)",
    "CREATE INDEX IF NOT EXISTS idx_sections_project_id ON sections(project_id)",
    "CREATE INDEX IF NOT EXISTS idx_cards_project_id ON cards(project_id)",
    "CREATE INDEX IF NOT EXISTS idx_cards_section_id ON cards(section_id)",
];

const DOWN_STATEMENTS: &[&str] = &[
    "DROP INDEX IF EXISTS idx_cards_section_id",
    "DROP INDEX IF EXISTS idx_cards_project_id",
    "DROP INDEX IF EXISTS idx_sections_project_id",
    "DROP TABLE IF EXISTS cards",
    "DROP TABLE IF EXISTS sections",
    "DROP TABLE IF EXISTS projects",
];

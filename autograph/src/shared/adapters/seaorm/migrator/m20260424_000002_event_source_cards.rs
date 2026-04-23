use crate::card::adapters::seaorm::history::serialize_history;
use crate::card::entity::CardHistory;
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::{ConnectionTrait, DbBackend, FromQueryResult, Statement};
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Debug, FromQueryResult)]
struct LegacyCard {
    id: String,
    title: String,
    description: String,
    deadline: Option<String>,
    project_id: Option<String>,
    section_id: Option<String>,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE cards ADD COLUMN deleted INTEGER NOT NULL DEFAULT 0")
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE TABLE IF NOT EXISTS card_history (card_id TEXT PRIMARY KEY REFERENCES cards(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED, items TEXT NOT NULL DEFAULT '[]')",
            )
            .await?;

        let cards = LegacyCard::find_by_statement(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT id, title, description, deadline, project_id, section_id FROM cards".to_owned(),
        ))
        .all(manager.get_connection())
        .await?;

        for card in cards {
            let id = Uuid::parse_str(&card.id).map_err(|err| {
                DbErr::Migration(format!("Failed to parse card id during migration: {err}"))
            })?;
            let mut history = vec![CardHistory::CreateCard {
                id,
                title: card.title,
            }];

            if !card.description.is_empty() {
                history.push(CardHistory::ChangeDescription {
                    description: card.description,
                });
            }

            if let Some(deadline) = card.deadline {
                let deadline = parse_sqlite_datetime(&deadline)?;
                history.push(CardHistory::ChangeDeadline {
                    deadline: Some(deadline),
                });
            }

            if let Some(project_id) = card.project_id {
                let project_id = Uuid::parse_str(&project_id).map_err(|err| {
                    DbErr::Migration(format!(
                        "Failed to parse card project id during migration: {err}"
                    ))
                })?;
                history.push(CardHistory::BindProject {
                    project_id: Some(project_id),
                });
            }

            if let Some(section_id) = card.section_id {
                let section_id = Uuid::parse_str(&section_id).map_err(|err| {
                    DbErr::Migration(format!(
                        "Failed to parse card section id during migration: {err}"
                    ))
                })?;
                history.push(CardHistory::BindSection {
                    section_id: Some(section_id),
                });
            }

            let items =
                serialize_history(&history).map_err(|err| DbErr::Migration(err.to_string()))?;
            manager
                .get_connection()
                .execute(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "INSERT INTO card_history (card_id, items) VALUES (?, ?)",
                    [card.id.into(), items.into()],
                ))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS card_history")
            .await?;
        Ok(())
    }
}

fn parse_sqlite_datetime(value: &str) -> Result<DateTime<Utc>, DbErr> {
    if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
        return Ok(parsed.with_timezone(&Utc));
    }

    if let Ok(parsed) = DateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.f %:z") {
        return Ok(parsed.with_timezone(&Utc));
    }

    if let Ok(parsed) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.f") {
        return Ok(parsed.and_utc());
    }

    Err(DbErr::Migration(format!(
        "Failed to parse card deadline during migration: {value}"
    )))
}

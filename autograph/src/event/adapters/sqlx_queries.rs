use crate::event::Event;
use crate::event::queries::EventQueries;
use crate::shared::adapters::database::sqlx_database::SqlxConnection;
use crate::shared::error::AppErr;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxEventQueries {
    conn: SqlxConnection,
}

impl SqlxEventQueries {
    pub fn new(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl EventQueries for SqlxEventQueries {
    async fn get_all_events(&self) -> Result<Vec<Event>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, date, title, description, project_id FROM events ORDER BY rowid",
        )
        .fetch_all(&self.conn)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Event {
                id: row.get(0),
                date: row.get(1),
                title: row.get(2),
                description: row.get(3),
                project_id: row.get(4),
            })
            .collect())
    }

    async fn get_events_without_project(&self) -> Result<Vec<Event>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, date, title, description, project_id FROM events WHERE project_id IS NULL ORDER BY rowid",
        )
        .fetch_all(&self.conn)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Event {
                id: row.get(0),
                date: row.get(1),
                title: row.get(2),
                description: row.get(3),
                project_id: row.get(4),
            })
            .collect())
    }

    async fn get_events_by_project(&self, project_id: Uuid) -> Result<Vec<Event>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, date, title, description, project_id FROM events WHERE project_id = ?1 ORDER BY rowid",
        )
        .bind(project_id)
        .fetch_all(&self.conn)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| Event {
                id: row.get(0),
                date: row.get(1),
                title: row.get(2),
                description: row.get(3),
                project_id: row.get(4),
            })
            .collect())
    }
}

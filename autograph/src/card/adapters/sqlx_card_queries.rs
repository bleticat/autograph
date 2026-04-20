use crate::shared::adapters::database::sqlx_database::SqlxConnection;
use crate::shared::error::AppErr;
use crate::card::Card;
use crate::card::ports::card_queries::CardQueries;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxCardQueries {
    conn: SqlxConnection,
}

impl SqlxCardQueries {
    pub fn new(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl CardQueries for SqlxCardQueries {
    async fn get_all_cards(&self) -> Result<Vec<Card>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id FROM cards ORDER BY rowid",
        )
        .fetch_all(&self.conn)
        .await?;
        let cards = rows
            .into_iter()
            .map(|row| Card {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                completed: row.get::<bool, _>(4),
                project_id: row.get(5),
            })
            .collect();
        Ok(cards)
    }

    async fn get_cards_without_project(&self) -> Result<Vec<Card>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id FROM cards WHERE project_id IS NULL ORDER BY rowid",
        )
        .fetch_all(&self.conn)
        .await?;
        let cards = rows
            .into_iter()
            .map(|row| Card {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                completed: row.get::<bool, _>(4),
                project_id: row.get(5),
            })
            .collect();
        Ok(cards)
    }

    async fn get_cards_by_project(&self, project_id: Uuid) -> Result<Vec<Card>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id FROM cards WHERE project_id = ?1 ORDER BY rowid",
        )
        .bind(project_id)
        .fetch_all(&self.conn)
        .await?;
        let cards = rows
            .into_iter()
            .map(|row| Card {
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                deadline: row.get(3),
                completed: row.get::<bool, _>(4),
                project_id: row.get(5),
            })
            .collect();
        Ok(cards)
    }
}

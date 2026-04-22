use crate::section::entity::Section;
use crate::section::queries::SectionQueries;
use crate::shared::adapters::sqlx::database::SqlxConnection;
use crate::shared::error::AppErr;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxSectionQueries {
    conn: SqlxConnection,
}

impl SqlxSectionQueries {
    pub fn new(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl SectionQueries for SqlxSectionQueries {
    async fn get_sections_by_project(&self, project_id: Uuid) -> Result<Vec<Section>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, title, project_id FROM sections WHERE project_id = ?1 ORDER BY rowid",
        )
        .bind(project_id)
        .fetch_all(&self.conn)
        .await?;

        let sections = rows
            .into_iter()
            .map(|row| Section {
                id: row.get(0),
                title: row.get(1),
                project_id: row.get(2),
            })
            .collect();

        Ok(sections)
    }
}

use crate::section::entity::Section;
use crate::section::queries::SectionQueries;
use crate::shared::adapters::sqlx::database::SqlxConnection;
use crate::shared::error::AppErr;
use crate::shared::filter::QueryFilter;
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
    async fn filter(
        &self,
        limit: u32,
        offset: u32,
        project_id: QueryFilter<Uuid>,
    ) -> Result<Vec<Section>, AppErr> {
        let rows = match project_id {
            QueryFilter::Val(project_id) => {
                sqlx::query(
                    "SELECT id, title, project_id FROM sections WHERE project_id = ?1 ORDER BY rowid LIMIT ?2 OFFSET ?3",
                )
                .bind(project_id)
                .bind(i64::from(limit))
                .bind(i64::from(offset))
                .fetch_all(&self.conn)
                .await?
            }
            QueryFilter::None => {
                sqlx::query(
                    "SELECT id, title, project_id FROM sections WHERE project_id IS NULL ORDER BY rowid LIMIT ?1 OFFSET ?2",
                )
                .bind(i64::from(limit))
                .bind(i64::from(offset))
                .fetch_all(&self.conn)
                .await?
            }
            QueryFilter::Ignore => {
                sqlx::query(
                    "SELECT id, title, project_id FROM sections ORDER BY rowid LIMIT ?1 OFFSET ?2",
                )
                .bind(i64::from(limit))
                .bind(i64::from(offset))
                .fetch_all(&self.conn)
                .await?
            }
        };

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

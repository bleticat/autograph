use crate::projects::Project;
use crate::projects::ports::project_queries::ProjectQueries;
use crate::shared::adapters::database::sqlx_database::SqlxConnection;
use crate::shared::error::AppErr;
use sqlx::Row;

pub struct SqlxProjectQueries {
    conn: SqlxConnection,
}

impl SqlxProjectQueries {
    pub fn new(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl ProjectQueries for SqlxProjectQueries {
    async fn get_all_projects(&self) -> Result<Vec<Project>, AppErr> {
        let rows = sqlx::query("SELECT id, title FROM projects ORDER BY rowid")
            .fetch_all(&self.conn)
            .await?;
        let projects = rows
            .into_iter()
            .map(|row| Project {
                id: row.get(0),
                title: row.get(1),
            })
            .collect();
        Ok(projects)
    }
}

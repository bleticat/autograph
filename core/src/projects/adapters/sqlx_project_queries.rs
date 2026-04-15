use crate::projects::ports::project_queries::ProjectQueries;
use crate::projects::Project;
use crate::shared::adapters::database::sqlx_database::SqlxConnection;
use crate::shared::error::AppErr;
use sqlx::Row;

pub struct SqliteProjectQueries {
    conn: SqlxConnection,
}

impl From<SqlxConnection> for SqliteProjectQueries {
    fn from(conn: SqlxConnection) -> Self {
        Self { conn }
    }
}

impl ProjectQueries for SqliteProjectQueries {
    type Conn = SqlxConnection;

    async fn get_all_projects(&self) -> Result<Vec<Project>, AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        let rows = sqlx::query("SELECT id, title FROM projects ORDER BY rowid")
            .fetch_all(&mut *conn)
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

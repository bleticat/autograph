use crate::projects::ports::project_queries::ProjectQueries;
use crate::projects::Project;
use crate::shared::adapters::rustqlite_database::RustqliteConnection;
use crate::shared::error::AppErr;
use futures::executor::block_on;
use sqlx::Row;

pub struct SqliteProjectQueries {
    conn: RustqliteConnection,
}

impl From<RustqliteConnection> for SqliteProjectQueries {
    fn from(conn: RustqliteConnection) -> Self {
        Self { conn }
    }
}

impl ProjectQueries for SqliteProjectQueries {
    type Conn = RustqliteConnection;

    fn get_all_projects(&self) -> Result<Vec<Project>, AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        let rows = block_on(
            sqlx::query("SELECT id, title FROM projects ORDER BY rowid").fetch_all(&mut *conn),
        )?;
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

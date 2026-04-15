use crate::projects::ports::project_queries::ProjectQueries;
use crate::projects::Project;
use crate::shared::adapters::rustqlite_database::RustqliteConnection;
use crate::shared::error::AppErr;

pub struct SqliteProjectQueries<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<RustqliteConnection<'a>> for SqliteProjectQueries<'a> {
    fn from(conn: RustqliteConnection<'a>) -> Self {
        Self { conn: conn.raw() }
    }
}

impl<'a> ProjectQueries for SqliteProjectQueries<'a> {
    type Conn = RustqliteConnection<'a>;

    fn get_all_projects(&self) -> Result<Vec<Project>, AppErr> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title FROM projects ORDER BY rowid")?;
        let projects = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    title: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(projects)
    }
}

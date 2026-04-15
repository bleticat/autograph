use crate::projects::ports::project_repo::ProjectRepository;
use crate::projects::Project;
use crate::shared::adapters::rustqlite_database::RustqliteTransaction;
use crate::shared::error::AppErr;

pub struct SqliteProjectRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<RustqliteTransaction<'a>> for SqliteProjectRepository<'a> {
    fn from(tx: RustqliteTransaction<'a>) -> Self {
        Self { conn: tx.raw() }
    }
}

impl<'a> ProjectRepository for SqliteProjectRepository<'a> {
    type Tx = RustqliteTransaction<'a>;

    fn get(&self, id: i64) -> Result<Option<Project>, AppErr> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title FROM projects WHERE id = ?1")?;
        let mut rows = stmt.query([id])?;
        match rows.next()? {
            Some(row) => Ok(Some(Project {
                id: row.get(0)?,
                title: row.get(1)?,
            })),
            None => Ok(None),
        }
    }

    fn save(&self, project: &Project) -> Result<i64, AppErr> {
        if project.id == 0 {
            self.conn
                .execute("INSERT INTO projects (title) VALUES (?1)", [&project.title])?;
            Ok(self.conn.last_insert_rowid())
        } else {
            self.conn.execute(
                "UPDATE projects SET title = ?1 WHERE id = ?2",
                rusqlite::params![project.title, project.id],
            )?;
            Ok(project.id)
        }
    }

    fn delete(&self, id: i64) -> Result<(), AppErr> {
        self.conn
            .execute("DELETE FROM projects WHERE id = ?1", [id])?;
        Ok(())
    }
}

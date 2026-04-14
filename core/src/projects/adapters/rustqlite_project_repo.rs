use crate::projects::ports::project_repo::ProjectRepository;
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

    fn add(&self, title: &str) -> Result<i64, AppErr> {
        self.conn
            .execute("INSERT INTO projects (title) VALUES (?1)", [title])?;
        Ok(self.conn.last_insert_rowid())
    }
}

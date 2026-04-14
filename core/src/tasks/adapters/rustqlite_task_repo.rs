use crate::shared::adapters::rustqlite_database::RustqliteTransaction;
use crate::shared::error::AppErr;
use crate::tasks::ports::task_repo::TodoRepository;

pub struct SqliteTodoRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<RustqliteTransaction<'a>> for SqliteTodoRepository<'a> {
    fn from(tx: RustqliteTransaction<'a>) -> Self {
        Self { conn: tx.raw() }
    }
}

impl<'a> TodoRepository for SqliteTodoRepository<'a> {
    type Tx = RustqliteTransaction<'a>;

    fn add(&self, title: &str) -> Result<i64, AppErr> {
        self.conn
            .execute("INSERT INTO todos (title) VALUES (?1)", [title])?;
        Ok(self.conn.last_insert_rowid())
    }

    fn add_with_project(&self, title: &str, project_id: i64) -> Result<i64, AppErr> {
        self.conn.execute(
            "INSERT INTO todos (title, project_id) VALUES (?1, ?2)",
            rusqlite::params![title, project_id],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    fn toggle(&self, id: i64) -> Result<(), AppErr> {
        self.conn.execute(
            "UPDATE todos SET completed = 1 - completed WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    fn delete(&self, id: i64) -> Result<(), AppErr> {
        self.conn
            .execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }
}

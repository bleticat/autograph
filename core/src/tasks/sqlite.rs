use super::ports::{TaskQueries, TodoRepository};
use super::Todo;
use crate::shared::error::AppErr;
use crate::shared::sqlite::{SqliteConnection, SqliteTransaction};

pub struct SqliteTaskQueries<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<SqliteConnection<'a>> for SqliteTaskQueries<'a> {
    fn from(conn: SqliteConnection<'a>) -> Self {
        Self { conn: conn.raw() }
    }
}

impl<'a> TaskQueries for SqliteTaskQueries<'a> {
    type Conn = SqliteConnection<'a>;

    fn get_all_todos(&self) -> Result<Vec<Todo>, AppErr> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed FROM todos ORDER BY id")?;
        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i32>(2)? != 0,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(todos)
    }
}

pub struct SqliteTodoRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<SqliteTransaction<'a>> for SqliteTodoRepository<'a> {
    fn from(tx: SqliteTransaction<'a>) -> Self {
        Self { conn: tx.raw() }
    }
}

impl<'a> TodoRepository for SqliteTodoRepository<'a> {
    type Tx = SqliteTransaction<'a>;

    fn add(&self, title: &str) -> Result<i64, AppErr> {
        self.conn
            .execute("INSERT INTO todos (title) VALUES (?1)", [title])?;
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

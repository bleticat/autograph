use super::ports::{TaskQueries, TodoRepository};
use super::Todo;
use crate::shared::sqlite::{SqliteConnection, SqliteTransaction};

pub struct SqliteTaskQueries<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SqliteTaskQueries<'a> {
    pub fn new(conn: SqliteConnection<'a>) -> Self {
        Self { conn: conn.0 }
    }
}

impl TaskQueries for SqliteTaskQueries<'_> {
    fn get_all_todos(&self) -> Result<Vec<Todo>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, completed FROM todos ORDER BY id")
            .map_err(|e| e.to_string())?;
        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i32>(2)? != 0,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(todos)
    }
}

pub struct SqliteTodoRepository<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SqliteTodoRepository<'a> {
    pub fn new(tx: &'a SqliteTransaction<'_>) -> Self {
        Self { conn: tx.0 }
    }
}

impl TodoRepository for SqliteTodoRepository<'_> {
    fn add(&self, title: &str) -> Result<i64, String> {
        self.conn
            .execute("INSERT INTO todos (title) VALUES (?1)", [title])
            .map_err(|e| e.to_string())?;
        Ok(self.conn.last_insert_rowid())
    }

    fn toggle(&self, id: i64) -> Result<(), String> {
        self.conn
            .execute(
                "UPDATE todos SET completed = NOT completed WHERE id = ?1",
                [id],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn delete(&self, id: i64) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM todos WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

use crate::Todo;
use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init(conn)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todos (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )?;
        Ok(Self { conn })
    }

    pub fn get_all(&self) -> Result<Vec<Todo>> {
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
            .collect::<Result<Vec<_>>>()?;
        Ok(todos)
    }

    pub fn add(&self, title: &str) -> Result<()> {
        self.conn
            .execute("INSERT INTO todos (title) VALUES (?1)", [title])?;
        Ok(())
    }

    pub fn toggle(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE todos SET completed = NOT completed WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;
        Ok(())
    }
}

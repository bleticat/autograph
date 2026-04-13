use crate::ports::TodoRepository;
use crate::Todo;
use rusqlite::Connection;

pub struct SqliteTodoRepository {
    conn: Connection,
}

impl SqliteTodoRepository {
    pub fn open(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| e.to_string())?;
        Self::init(conn)
    }

    pub fn in_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> Result<Self, String> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS todos (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );",
        )
        .map_err(|e| e.to_string())?;
        Ok(Self { conn })
    }
}

impl TodoRepository for SqliteTodoRepository {
    fn get_all(&self) -> Result<Vec<Todo>, String> {
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

    fn add(&self, title: &str) -> Result<(), String> {
        self.conn
            .execute("INSERT INTO todos (title) VALUES (?1)", [title])
            .map_err(|e| e.to_string())?;
        Ok(())
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

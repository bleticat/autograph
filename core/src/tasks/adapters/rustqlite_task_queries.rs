use crate::shared::adapters::rustqlite_database::RustqliteConnection;
use crate::shared::error::AppErr;
use crate::tasks::ports::task_queries::TaskQueries;
use crate::tasks::Todo;

pub struct SqliteTaskQueries<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> From<RustqliteConnection<'a>> for SqliteTaskQueries<'a> {
    fn from(conn: RustqliteConnection<'a>) -> Self {
        Self { conn: conn.raw() }
    }
}

impl<'a> TaskQueries for SqliteTaskQueries<'a> {
    type Conn = RustqliteConnection<'a>;

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

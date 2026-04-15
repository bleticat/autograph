use crate::shared::adapters::rustqlite_database::RustqliteConnection;
use crate::shared::error::AppErr;
use crate::tasks::ports::task_queries::TaskQueries;
use crate::tasks::Todo;
use uuid::Uuid;

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
            .prepare("SELECT id, title, completed, project_id FROM todos ORDER BY rowid")?;
        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i32>(2)? != 0,
                    project_id: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(todos)
    }

    fn get_todos_without_project(&self) -> Result<Vec<Todo>, AppErr> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, completed, project_id FROM todos WHERE project_id IS NULL ORDER BY rowid",
        )?;
        let todos = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i32>(2)? != 0,
                    project_id: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(todos)
    }

    fn get_todos_by_project(&self, project_id: Uuid) -> Result<Vec<Todo>, AppErr> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, completed, project_id FROM todos WHERE project_id = ?1 ORDER BY rowid",
        )?;
        let todos = stmt
            .query_map([project_id], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get::<_, i32>(2)? != 0,
                    project_id: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(todos)
    }
}

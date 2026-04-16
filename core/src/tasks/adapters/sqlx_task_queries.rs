use crate::shared::adapters::database::sqlx_database::SqlxConn;
use crate::shared::error::AppErr;
use crate::shared::ports::queries::Queries;
use crate::tasks::ports::task_queries::TaskQueries;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

pub struct SqliteTaskQueries {
    conn: SqlxConn,
}

impl Queries for SqliteTaskQueries {
    type Conn = SqlxConn;

    fn bind(conn: SqlxConn) -> Self {
        Self { conn }
    }
}

impl TaskQueries for SqliteTaskQueries {
    async fn get_all_todos(&self) -> Result<Vec<Todo>, AppErr> {
        let rows = sqlx::query("SELECT id, title, completed, project_id FROM todos ORDER BY rowid")
            .fetch_all(&self.conn)
            .await?;
        let todos = rows
            .into_iter()
            .map(|row| Todo {
                id: row.get(0),
                title: row.get(1),
                completed: row.get::<bool, _>(2),
                project_id: row.get(3),
            })
            .collect();
        Ok(todos)
    }

    async fn get_todos_without_project(&self) -> Result<Vec<Todo>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, title, completed, project_id FROM todos WHERE project_id IS NULL ORDER BY rowid",
        )
        .fetch_all(&self.conn)
        .await?;
        let todos = rows
            .into_iter()
            .map(|row| Todo {
                id: row.get(0),
                title: row.get(1),
                completed: row.get::<bool, _>(2),
                project_id: row.get(3),
            })
            .collect();
        Ok(todos)
    }

    async fn get_todos_by_project(&self, project_id: Uuid) -> Result<Vec<Todo>, AppErr> {
        let rows = sqlx::query(
            "SELECT id, title, completed, project_id FROM todos WHERE project_id = ?1 ORDER BY rowid",
        )
        .bind(project_id)
        .fetch_all(&self.conn)
        .await?;
        let todos = rows
            .into_iter()
            .map(|row| Todo {
                id: row.get(0),
                title: row.get(1),
                completed: row.get::<bool, _>(2),
                project_id: row.get(3),
            })
            .collect();
        Ok(todos)
    }
}

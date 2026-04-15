use crate::shared::adapters::database::sqlx_database::SqlxTransaction;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::ports::task_repo::TodoRepository;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

pub struct SqliteTodoRepository {
    conn: SqlxTransaction,
}

impl From<SqlxTransaction> for SqliteTodoRepository {
    fn from(tx: SqlxTransaction) -> Self {
        Self { conn: tx }
    }
}

impl Repository<Todo> for SqliteTodoRepository {
    type Tx = SqlxTransaction;

    async fn get(&self, id: Uuid) -> Result<Option<Todo>, AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        let row = sqlx::query("SELECT id, title, completed, project_id FROM todos WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?;
        Ok(row.map(|row| Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get::<bool, _>(2),
            project_id: row.get(3),
        }))
    }

    async fn save(&self, todo: &Todo) -> Result<Uuid, AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        if todo.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO todos (id, title, completed, project_id) VALUES (?1, ?2, ?3, ?4)",
            )
            .bind(id)
            .bind(&todo.title)
            .bind(todo.completed)
            .bind(todo.project_id)
            .execute(&mut *conn)
            .await?;
            Ok(id)
        } else {
            let updated = sqlx::query(
                "UPDATE todos SET title = ?1, completed = ?2, project_id = ?3 WHERE id = ?4",
            )
            .bind(&todo.title)
            .bind(todo.completed)
            .bind(todo.project_id)
            .bind(todo.id)
            .execute(&mut *conn)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(todo.id)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        let conn = self.conn.raw();
        let mut conn = conn.borrow_mut();
        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }
}

impl TodoRepository for SqliteTodoRepository {}

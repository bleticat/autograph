use crate::shared::adapters::database::sqlx_database::SqlxTransaction;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

pub struct SqliteTodoRepository<'a> {
    conn: &'a SqlxTransaction,
}

impl<'a> Repository<'a, Todo> for SqliteTodoRepository<'a> {
    type Tx = SqlxTransaction;

    fn bind(tx: &'a Self::Tx) -> Self {
        Self { conn: tx }
    }

    async fn get(&self, id: Uuid) -> Result<Option<Todo>, AppErr> {
        let mut tx = self.conn.acquire().await;
        let row = sqlx::query("SELECT id, title, completed, project_id FROM todos WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?;
        Ok(row.map(|row| Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get::<bool, _>(2),
            project_id: row.get(3),
        }))
    }

    async fn save(&self, todo: &Todo) -> Result<Uuid, AppErr> {
        let mut tx = self.conn.acquire().await;
        if todo.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO todos (id, title, completed, project_id) VALUES (?1, ?2, ?3, ?4)",
            )
            .bind(id)
            .bind(&todo.title)
            .bind(todo.completed)
            .bind(todo.project_id)
            .execute(&mut **tx)
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
            .execute(&mut **tx)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(todo.id)
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        let mut tx = self.conn.acquire().await;
        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}

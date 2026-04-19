use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxTodoRepository<'a> {
    tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl<'a> SqlxTodoRepository<'a> {
    pub fn new(tx: &'a mut sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl<'a> Repository<Todo> for SqlxTodoRepository<'a> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Todo>, AppErr> {
        let row = sqlx::query("SELECT id, title, completed, project_id FROM todos WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut **self.tx)
            .await?;
        Ok(row.map(|row| Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get::<bool, _>(2),
            project_id: row.get(3),
        }))
    }

    async fn save(&mut self, todo: Todo) -> Result<Todo, AppErr> {
        if todo.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO todos (id, title, completed, project_id) VALUES (?1, ?2, ?3, ?4)",
            )
            .bind(id)
            .bind(&todo.title)
            .bind(todo.completed)
            .bind(todo.project_id)
            .execute(&mut **self.tx)
            .await?;
            Ok(Todo { id, ..todo })
        } else {
            let updated = sqlx::query(
                "UPDATE todos SET title = ?1, completed = ?2, project_id = ?3 WHERE id = ?4",
            )
            .bind(&todo.title)
            .bind(todo.completed)
            .bind(todo.project_id)
            .bind(todo.id)
            .execute(&mut **self.tx)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(todo)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&mut **self.tx)
            .await?;
        Ok(())
    }
}

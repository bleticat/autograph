use crate::shared::adapters::database::sqlx_unit_of_work::SqlxUnitOfWork;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

impl Repository<Todo> for SqlxUnitOfWork {
    async fn get(&mut self, id: Uuid) -> Result<Option<Todo>, AppErr> {
        let tx = self.tx();
        let row = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id FROM todos WHERE id = ?1",
        )
            .bind(id)
            .fetch_optional(&mut **tx)
            .await?;
        Ok(row.map(|row| Todo {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            deadline: row.get(3),
            completed: row.get::<bool, _>(4),
            project_id: row.get(5),
        }))
    }

    async fn save(&mut self, todo: Todo) -> Result<Todo, AppErr> {
        if todo.id.is_nil() {
            let id = Uuid::new_v4();
            let tx = self.tx();
            sqlx::query(
                "INSERT INTO todos (id, title, description, deadline, completed, project_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )
            .bind(id)
            .bind(&todo.title)
            .bind(&todo.description)
            .bind(&todo.deadline)
            .bind(todo.completed)
            .bind(todo.project_id)
            .execute(&mut **tx)
            .await?;
            Ok(Todo { id, ..todo })
        } else {
            let tx = self.tx();
            let updated = sqlx::query(
                "UPDATE todos SET title = ?1, description = ?2, deadline = ?3, completed = ?4, project_id = ?5 WHERE id = ?6",
            )
            .bind(&todo.title)
            .bind(&todo.description)
            .bind(&todo.deadline)
            .bind(todo.completed)
            .bind(todo.project_id)
            .bind(todo.id)
            .execute(&mut **tx)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(todo)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        let tx = self.tx();
        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }
}

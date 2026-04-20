use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

impl Repository for Todo {
    type Tx = sqlx::Transaction<'static, sqlx::Sqlite>;

    async fn get<U>(uow: &mut U, id: Uuid) -> Result<Option<Todo>, AppErr>
    where
        U: UnitOfWork<Tx = Self::Tx>,
    {
        let row = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id FROM todos WHERE id = ?1",
        )
            .bind(id)
            .fetch_optional(&mut **uow.tx())
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

    async fn save<U>(self, uow: &mut U) -> Result<Todo, AppErr>
    where
        U: UnitOfWork<Tx = Self::Tx>,
    {
        let todo = self;
        if todo.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO todos (id, title, description, deadline, completed, project_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )
            .bind(id)
            .bind(&todo.title)
            .bind(&todo.description)
            .bind(&todo.deadline)
            .bind(todo.completed)
            .bind(todo.project_id)
            .execute(&mut **uow.tx())
            .await?;
            Ok(Todo { id, ..todo })
        } else {
            let updated = sqlx::query(
                "UPDATE todos SET title = ?1, description = ?2, deadline = ?3, completed = ?4, project_id = ?5 WHERE id = ?6",
            )
            .bind(&todo.title)
            .bind(&todo.description)
            .bind(&todo.deadline)
            .bind(todo.completed)
            .bind(todo.project_id)
            .bind(todo.id)
            .execute(&mut **uow.tx())
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(todo)
        }
    }

    async fn delete<U>(uow: &mut U, id: Uuid) -> Result<(), AppErr>
    where
        U: UnitOfWork<Tx = Self::Tx>,
    {
        sqlx::query("DELETE FROM todos WHERE id = ?1")
            .bind(id)
            .execute(&mut **uow.tx())
            .await?;
        Ok(())
    }
}

use crate::events::Event;
use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::Todo;
use sqlx::Row;
use uuid::Uuid;

pub struct SqlxUnitOfWork {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl SqlxUnitOfWork {
    pub(super) fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl UnitOfWork for SqlxUnitOfWork {
    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

impl Repository<Project> for SqlxUnitOfWork {
    async fn get(&mut self, id: Uuid) -> Result<Option<Project>, AppErr> {
        let row = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut *self.tx)
            .await?;
        Ok(row.map(|row| Project {
            id: row.get(0),
            title: row.get(1),
        }))
    }

    async fn save(&mut self, project: Project) -> Result<Project, AppErr> {
        if project.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query("INSERT INTO projects (id, title) VALUES (?1, ?2)")
                .bind(id)
                .bind(project.title.as_str())
                .execute(&mut *self.tx)
                .await?;
            Ok(Project { id, ..project })
        } else {
            sqlx::query("UPDATE projects SET title = ?1 WHERE id = ?2")
                .bind(project.title.as_str())
                .bind(project.id)
                .execute(&mut *self.tx)
                .await?;
            Ok(project)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        sqlx::query("DELETE FROM projects WHERE id = ?1")
            .bind(id)
            .execute(&mut *self.tx)
            .await?;
        Ok(())
    }
}

impl Repository<Todo> for SqlxUnitOfWork {
    async fn get(&mut self, id: Uuid) -> Result<Option<Todo>, AppErr> {
        let row = sqlx::query(
            "SELECT id, title, description, deadline, completed, project_id FROM todos WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(&mut *self.tx)
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
            sqlx::query(
                "INSERT INTO todos (id, title, description, deadline, completed, project_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )
            .bind(id)
            .bind(&todo.title)
            .bind(&todo.description)
            .bind(&todo.deadline)
            .bind(todo.completed)
            .bind(todo.project_id)
            .execute(&mut *self.tx)
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
            .execute(&mut *self.tx)
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
            .execute(&mut *self.tx)
            .await?;
        Ok(())
    }
}

impl Repository<Event> for SqlxUnitOfWork {
    async fn get(&mut self, id: Uuid) -> Result<Option<Event>, AppErr> {
        let row = sqlx::query("SELECT id, date, title, description, project_id FROM events WHERE id = ?1")
            .bind(id)
            .fetch_optional(&mut *self.tx)
            .await?;
        Ok(row.map(|row| Event {
            id: row.get(0),
            date: row.get(1),
            title: row.get(2),
            description: row.get(3),
            project_id: row.get(4),
        }))
    }

    async fn save(&mut self, event: Event) -> Result<Event, AppErr> {
        if event.id.is_nil() {
            let id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO events (id, date, title, description, project_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            )
            .bind(id)
            .bind(event.date)
            .bind(&event.title)
            .bind(&event.description)
            .bind(event.project_id)
            .execute(&mut *self.tx)
            .await?;
            Ok(Event { id, ..event })
        } else {
            let updated = sqlx::query(
                "UPDATE events SET date = ?1, title = ?2, description = ?3, project_id = ?4 WHERE id = ?5",
            )
            .bind(event.date)
            .bind(&event.title)
            .bind(&event.description)
            .bind(event.project_id)
            .bind(event.id)
            .execute(&mut *self.tx)
            .await?
            .rows_affected();
            if updated == 0 {
                return Err(sqlx::Error::RowNotFound.into());
            }
            Ok(event)
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        sqlx::query("DELETE FROM events WHERE id = ?1")
            .bind(id)
            .execute(&mut *self.tx)
            .await?;
        Ok(())
    }
}

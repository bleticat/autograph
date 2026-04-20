use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::Todo;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct TaskCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> TaskCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Todo, AppErr> {
        <U as Repository<Todo>>::save(
            self.uow,
            Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: None,
            },
        )
        .await
    }

    pub async fn add_with_project(
        &mut self,
        title: &str,
        project_id: Uuid,
    ) -> Result<Todo, AppErr> {
        <U as Repository<Todo>>::save(
            self.uow,
            Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: Some(project_id),
            },
        )
        .await
    }

    pub async fn edit(
        &mut self,
        id: Uuid,
        title: &str,
        description: &str,
        deadline: Option<OffsetDateTime>,
    ) -> Result<(), AppErr> {
        let todo = <U as Repository<Todo>>::get(self.uow, id).await?;
        if let Some(mut todo) = todo {
            todo.title = title.to_owned();
            todo.description = description.to_owned();
            todo.deadline = deadline;
            <U as Repository<Todo>>::save(self.uow, todo).await?;
        }
        Ok(())
    }

    pub async fn toggle(&mut self, id: Uuid) -> Result<(), AppErr> {
        let todo = <U as Repository<Todo>>::get(self.uow, id).await?;
        if let Some(mut todo) = todo {
            todo.completed = !todo.completed;
            <U as Repository<Todo>>::save(self.uow, todo).await?;
        }
        Ok(())
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        <U as Repository<Todo>>::delete(self.uow, id).await
    }
}

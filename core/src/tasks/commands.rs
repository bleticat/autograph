use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::Todo;
use uuid::Uuid;

pub struct TaskCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> TaskCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Todo, AppErr> {
        self.uow
            .tasks()
            .save(Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: None,
            })
            .await
    }

    pub async fn add_with_project(
        &mut self,
        title: &str,
        project_id: Uuid,
    ) -> Result<Todo, AppErr> {
        self.uow
            .tasks()
            .save(Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: Some(project_id),
            })
            .await
    }

    pub async fn edit(
        &mut self,
        id: Uuid,
        title: &str,
        description: &str,
        deadline: Option<String>,
    ) -> Result<(), AppErr> {
        let todo = self.uow.tasks().get(id).await?;
        if let Some(mut todo) = todo {
            todo.title = title.to_owned();
            todo.description = description.to_owned();
            todo.deadline = deadline;
            self.uow.tasks().save(todo).await?;
        }
        Ok(())
    }

    pub async fn toggle(&mut self, id: Uuid) -> Result<(), AppErr> {
        let todo = self.uow.tasks().get(id).await?;
        if let Some(mut todo) = todo {
            todo.completed = !todo.completed;
            self.uow.tasks().save(todo).await?;
        }
        Ok(())
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        self.uow.tasks().delete(id).await
    }
}

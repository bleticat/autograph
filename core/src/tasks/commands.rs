use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use uuid::Uuid;

pub struct TaskCommands<R: Repository<Todo>> {
    repo: R,
}

impl<R: Repository<Todo>> TaskCommands<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn add(&mut self, title: &str) -> Result<Todo, AppErr> {
        self.repo
            .save(Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
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
        self.repo
            .save(Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                completed: false,
                project_id: Some(project_id),
            })
            .await
    }

    pub async fn toggle(&mut self, id: Uuid) -> Result<(), AppErr> {
        let todo = self.repo.get(id).await?;
        if let Some(mut todo) = todo {
            todo.completed = !todo.completed;
            self.repo.save(todo).await?;
        }
        Ok(())
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        self.repo.delete(id).await
    }
}

use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use uuid::Uuid;

pub struct TaskCommands<'a, R: Repository<Todo>> {
    tasks: &'a R,
}

impl<'a, R: Repository<Todo>> TaskCommands<'a, R> {
    pub fn new(tasks: &'a R) -> Self {
        Self { tasks }
    }

    pub async fn add(&self, title: &str) -> Result<Todo, AppErr> {
        self.tasks
            .save(Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                completed: false,
                project_id: None,
            })
            .await
    }

    pub async fn add_with_project(&self, title: &str, project_id: Uuid) -> Result<Todo, AppErr> {
        self.tasks
            .save(Todo {
                id: Uuid::nil(),
                title: title.to_owned(),
                completed: false,
                project_id: Some(project_id),
            })
            .await
    }

    pub async fn toggle(&self, id: Uuid) -> Result<(), AppErr> {
        if let Some(mut todo) = self.tasks.get(id).await? {
            todo.completed = !todo.completed;
            self.tasks.save(todo).await?;
        }
        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        self.tasks.delete(id).await
    }
}

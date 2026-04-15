use super::ports::task_repo::TodoRepository;
use crate::shared::error::AppErr;
use crate::tasks::Todo;
use uuid::Uuid;

pub struct TaskCommands<'a, R: TodoRepository> {
    tasks: &'a R,
}

impl<'a, R: TodoRepository> TaskCommands<'a, R> {
    pub fn new(tasks: &'a R) -> Self {
        Self { tasks }
    }

    pub fn add(&self, title: &str) -> Result<Uuid, AppErr> {
        self.tasks.save(&Todo {
            id: Uuid::nil(),
            title: title.to_owned(),
            completed: false,
            project_id: None,
        })
    }

    pub fn add_with_project(&self, title: &str, project_id: Uuid) -> Result<Uuid, AppErr> {
        self.tasks.save(&Todo {
            id: Uuid::nil(),
            title: title.to_owned(),
            completed: false,
            project_id: Some(project_id),
        })
    }

    pub fn toggle(&self, id: Uuid) -> Result<(), AppErr> {
        if let Some(mut todo) = self.tasks.get(id)? {
            todo.completed = !todo.completed;
            self.tasks.save(&todo)?;
        }
        Ok(())
    }

    pub fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        self.tasks.delete(id)
    }
}

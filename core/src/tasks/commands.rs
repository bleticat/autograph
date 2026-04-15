use super::ports::task_repo::TodoRepository;
use crate::shared::error::AppErr;
use crate::tasks::Todo;

pub struct TaskCommands<'a, R: TodoRepository> {
    tasks: &'a R,
}

impl<'a, R: TodoRepository> TaskCommands<'a, R> {
    pub fn new(tasks: &'a R) -> Self {
        Self { tasks }
    }

    pub fn add(&self, title: &str) -> Result<i64, AppErr> {
        self.tasks.save(&Todo {
            id: 0,
            title: title.to_owned(),
            completed: false,
            project_id: None,
        })
    }

    pub fn add_with_project(&self, title: &str, project_id: i64) -> Result<i64, AppErr> {
        self.tasks.save(&Todo {
            id: 0,
            title: title.to_owned(),
            completed: false,
            project_id: Some(project_id),
        })
    }

    pub fn toggle(&self, id: i64) -> Result<(), AppErr> {
        if let Some(mut todo) = self.tasks.get(id)? {
            todo.completed = !todo.completed;
            self.tasks.save(&todo)?;
        }
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<(), AppErr> {
        self.tasks.delete(id)
    }
}

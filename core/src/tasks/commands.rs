use super::ports::task_repo::TodoRepository;
use crate::shared::error::AppErr;

pub struct TaskCommands<'a, R: TodoRepository> {
    tasks: &'a R,
}

impl<'a, R: TodoRepository> TaskCommands<'a, R> {
    pub fn new(tasks: &'a R) -> Self {
        Self { tasks }
    }

    pub fn add(&self, title: &str) -> Result<i64, AppErr> {
        self.tasks.add(title)
    }

    pub fn toggle(&self, id: i64) -> Result<(), AppErr> {
        self.tasks.toggle(id)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppErr> {
        self.tasks.delete(id)
    }
}

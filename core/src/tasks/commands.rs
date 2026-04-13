use super::ports::TodoRepository;
use crate::error::CoreError;

pub struct TaskCommands<'a, R: TodoRepository> {
    tasks: &'a R,
}

impl<'a, R: TodoRepository> TaskCommands<'a, R> {
    pub fn new(tasks: &'a R) -> Self {
        Self { tasks }
    }

    pub fn add(&self, title: &str) -> Result<i64, CoreError> {
        self.tasks.add(title)
    }

    pub fn toggle(&self, id: i64) -> Result<(), CoreError> {
        self.tasks.toggle(id)
    }

    pub fn delete(&self, id: i64) -> Result<(), CoreError> {
        self.tasks.delete(id)
    }
}

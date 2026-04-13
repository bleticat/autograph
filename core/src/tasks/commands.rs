use super::ports::TodoRepository;

pub struct TaskCommands<'a> {
    tasks: &'a dyn TodoRepository,
}

impl<'a> TaskCommands<'a> {
    pub fn new(tasks: &'a dyn TodoRepository) -> Self {
        Self { tasks }
    }

    pub fn add(&self, title: &str) -> Result<i64, String> {
        self.tasks.add(title)
    }

    pub fn toggle(&self, id: i64) -> Result<(), String> {
        self.tasks.toggle(id)
    }

    pub fn delete(&self, id: i64) -> Result<(), String> {
        self.tasks.delete(id)
    }
}

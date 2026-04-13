use crate::ports::TodoRepository;

pub fn add_todo(repo: &dyn TodoRepository, title: &str) -> Result<(), String> {
    repo.add(title)
}

pub fn toggle_todo(repo: &dyn TodoRepository, id: i64) -> Result<(), String> {
    repo.toggle(id)
}

pub fn delete_todo(repo: &dyn TodoRepository, id: i64) -> Result<(), String> {
    repo.delete(id)
}

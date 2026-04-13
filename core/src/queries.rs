use crate::ports::TodoRepository;
use crate::Todo;

pub fn get_all_todos(repo: &dyn TodoRepository) -> Result<Vec<Todo>, String> {
    repo.get_all()
}

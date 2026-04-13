use super::Todo;

pub trait TodoRepository {
    fn add(&self, title: &str) -> Result<i64, String>;
    fn toggle(&self, id: i64) -> Result<(), String>;
    fn delete(&self, id: i64) -> Result<(), String>;
}

pub trait TaskQueries {
    fn get_all_todos(&self) -> Result<Vec<Todo>, String>;
}

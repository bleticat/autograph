use crate::Todo;

pub trait TodoRepository {
    fn get_all(&self) -> Result<Vec<Todo>, String>;
    fn add(&self, title: &str) -> Result<(), String>;
    fn toggle(&self, id: i64) -> Result<(), String>;
    fn delete(&self, id: i64) -> Result<(), String>;
}

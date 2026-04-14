use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;
use crate::tasks::Todo;

pub trait TaskQueries: From<Self::Conn> {
    type Conn: Connection;
    fn get_all_todos(&self) -> Result<Vec<Todo>, AppErr>;
    fn get_todos_without_project(&self) -> Result<Vec<Todo>, AppErr>;
    fn get_todos_by_project(&self, project_id: i64) -> Result<Vec<Todo>, AppErr>;
}

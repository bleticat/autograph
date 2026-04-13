use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;
use crate::tasks::Todo;

pub trait TaskQueries: From<Self::Conn> {
    type Conn: Connection;
    fn get_all_todos(&self) -> Result<Vec<Todo>, AppErr>;
}

use crate::shared::error::AppErr;
use crate::shared::ports::Connection;
use crate::tasks::Todo;

/// A read-only query object for fetching todos.
///
/// Every implementor must also implement `From<Self::Conn>`, enforcing the
/// same idiomatic constructor convention as [`TodoRepository`].
pub trait TaskQueries: From<Self::Conn> {
    type Conn: Connection;
    fn get_all_todos(&self) -> Result<Vec<Todo>, AppErr>;
}

use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;
use crate::tasks::Todo;
use std::future::Future;
use uuid::Uuid;

pub trait TaskQueries: From<Self::Conn> {
    type Conn: Connection;
    fn get_all_todos(&self) -> impl Future<Output = Result<Vec<Todo>, AppErr>> + Send + '_;
    fn get_todos_without_project(&self) -> impl Future<Output = Result<Vec<Todo>, AppErr>> + Send + '_;
    fn get_todos_by_project(&self, project_id: Uuid) -> impl Future<Output = Result<Vec<Todo>, AppErr>> + Send + '_;
}

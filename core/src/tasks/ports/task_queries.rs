use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;
use crate::tasks::Todo;
use uuid::Uuid;

#[allow(async_fn_in_trait)]
pub trait TaskQueries: From<Self::Conn> {
    type Conn: Connection;
    async fn get_all_todos(&self) -> Result<Vec<Todo>, AppErr>;
    async fn get_todos_without_project(&self) -> Result<Vec<Todo>, AppErr>;
    async fn get_todos_by_project(&self, project_id: Uuid) -> Result<Vec<Todo>, AppErr>;
}

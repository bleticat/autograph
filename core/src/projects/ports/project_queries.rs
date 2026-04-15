use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;

#[allow(async_fn_in_trait)]
pub trait ProjectQueries: From<Self::Conn> {
    type Conn: Connection;
    async fn get_all_projects(&self) -> Result<Vec<Project>, AppErr>;
}

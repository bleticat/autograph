use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;

pub trait ProjectQueries: From<Self::Conn> {
    type Conn: Connection;
    fn get_all_projects(&self) -> Result<Vec<Project>, AppErr>;
}

use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Connection;
use std::future::Future;

pub trait ProjectQueries: From<Self::Conn> {
    type Conn: Connection;
    fn get_all_projects(&self) -> impl Future<Output = Result<Vec<Project>, AppErr>>;
}

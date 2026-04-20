use crate::project::entity::Project;
use crate::shared::error::AppErr;
use std::future::Future;

pub trait ProjectQueries {
    fn get_all_projects(&self) -> impl Future<Output = Result<Vec<Project>, AppErr>> + Send + '_;
}

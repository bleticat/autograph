use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::queries::Queries;
use std::future::Future;

pub trait ProjectQueries: Queries {
    fn get_all_projects(&self) -> impl Future<Output = Result<Vec<Project>, AppErr>> + Send + '_;
}

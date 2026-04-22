use crate::project::entity::{Project, ProjectData};
use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait ProjectQueries {
    fn filter(
        &self,
        limit: u32,
        offset: u32,
    ) -> impl Future<Output = Result<Vec<Project>, AppErr>> + Send + '_;
    fn get_project(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Option<ProjectData>, AppErr>> + Send + '_;

    fn get_all_projects(&self) -> impl Future<Output = Result<Vec<Project>, AppErr>> + Send + '_ {
        self.filter(u32::MAX, 0)
    }

    fn get_project_data(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Option<ProjectData>, AppErr>> + Send + '_ {
        self.get_project(project_id)
    }
}

use crate::section::entity::Section;
use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait SectionQueries {
    fn get_sections_by_project(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Section>, AppErr>> + Send + '_;
}

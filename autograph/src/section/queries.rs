use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::filter::QueryFilter;
use std::future::Future;
use uuid::Uuid;

pub trait SectionQueries {
    fn filter(
        &self,
        limit: u32,
        offset: u32,
        project_id: QueryFilter<Uuid>,
    ) -> impl Future<Output = Result<Vec<Section>, AppErr>> + Send;

    fn get_sections_by_project(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Section>, AppErr>> + Send {
        self.filter(u32::MAX, 0, QueryFilter::Val(project_id))
    }
}

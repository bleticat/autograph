use crate::event::entity::Event;
use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait EventQueries {
    fn get_all_events(&self) -> impl Future<Output = Result<Vec<Event>, AppErr>> + Send + '_;
    fn get_events_without_project(
        &self,
    ) -> impl Future<Output = Result<Vec<Event>, AppErr>> + Send + '_;
    fn get_events_by_project(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Event>, AppErr>> + Send + '_;
}

use crate::shared::error::AppErr;
use crate::card::entity::Card;
use std::future::Future;
use uuid::Uuid;

pub trait CardQueries {
    fn get_all_cards(&self) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send + '_;
    fn get_cards_without_project(
        &self,
    ) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send + '_;
    fn get_cards_by_project(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send + '_;
}

use crate::card::entity::CardHistory;
use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait CardHistoryRepository {
    fn get_history(
        &mut self,
        id: Uuid,
    ) -> impl Future<Output = Result<Vec<CardHistory>, AppErr>> + Send + '_;

    fn append_history(
        &mut self,
        id: Uuid,
        items: Vec<CardHistory>,
    ) -> impl Future<Output = Result<(), AppErr>> + Send + '_;
}

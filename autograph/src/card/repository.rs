use crate::card::entity::Card;
use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait CardEventRepository {
    fn get_by_section(
        &mut self,
        section_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send + '_;
}

use crate::card::entity::{Card, CardHistory};
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use std::future::Future;
use uuid::Uuid;

pub trait CardEventRepository {
    fn get_history(
        &mut self,
        id: Uuid,
    ) -> impl Future<Output = Result<Vec<CardHistory>, AppErr>> + Send + '_;

    fn append_history(
        &mut self,
        id: Uuid,
        items: Vec<CardHistory>,
    ) -> impl Future<Output = Result<(), AppErr>> + Send + '_;

    fn get_by_section(
        &mut self,
        section_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send + '_;
}

pub async fn rebuild_card_state<R>(repo: &mut R, id: Uuid) -> Result<Card, AppErr>
where
    R: Repository<Card> + CardEventRepository + Send,
{
    let history = repo.get_history(id).await?;
    let card = Card::apply(history)?;
    repo.save(card).await
}

pub async fn append_history_and_rebuild<R>(
    repo: &mut R,
    id: Uuid,
    items: Vec<CardHistory>,
) -> Result<Card, AppErr>
where
    R: Repository<Card> + CardEventRepository + Send,
{
    repo.append_history(id, items).await?;
    rebuild_card_state(repo, id).await
}

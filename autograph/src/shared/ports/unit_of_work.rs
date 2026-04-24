use crate::card::entity::Card;
use crate::card::history_repository::CardHistoryRepository;
use crate::card::repository::CardEventRepository;
use crate::project::entity::Project;
use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use std::future::Future;

pub trait UnitOfWork: Send {
    type ProjectRepository<'a>: Repository<Project> + Send
    where
        Self: 'a;
    type CardRepository<'a>: Repository<Card> + CardEventRepository + Send
    where
        Self: 'a;
    type CardHistoryRepository<'a>: CardHistoryRepository + Send
    where
        Self: 'a;
    type SectionRepository<'a>: Repository<Section> + Send
    where
        Self: 'a;

    fn project(&mut self) -> Self::ProjectRepository<'_>;
    fn card(&mut self) -> Self::CardRepository<'_>;
    fn card_history(&mut self) -> Self::CardHistoryRepository<'_>;
    fn section(&mut self) -> Self::SectionRepository<'_>;
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

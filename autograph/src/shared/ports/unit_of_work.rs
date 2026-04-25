use crate::card::entity::Card;
use crate::project::entity::Project;
use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use std::future::Future;

pub trait UnitOfWork: Send {
    // 'a is a generic associated type lifetime that lets the returned repository
    // borrow from &'a mut self; `where Self: 'a` guarantees self outlives 'a.
    type ProjectRepository<'a>: Repository<Project> + Send
    where
        Self: 'a;
    type CardRepository<'a>: Repository<Card> + Send
    where
        Self: 'a;
    type SectionRepository<'a>: Repository<Section> + Send
    where
        Self: 'a;

    fn project(&mut self) -> Self::ProjectRepository<'_>;
    fn card(&mut self) -> Self::CardRepository<'_>;
    fn section(&mut self) -> Self::SectionRepository<'_>;
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
    fn rollback(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

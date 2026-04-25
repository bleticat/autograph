use crate::card::entity::Card;
use crate::project::entity::Project;
use crate::section::entity::Section;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use std::future::Future;

pub trait UnitOfWork: Send {
    // Repository GAT lifetimes are the borrow of `self` used to create each
    // repository; the `Self: 'repo` bounds keep repositories scoped to the unit
    // of work they borrow from.
    type ProjectRepository<'repo>: Repository<Project> + Send
    where
        Self: 'repo;
    type CardRepository<'repo>: Repository<Card> + Send
    where
        Self: 'repo;
    type SectionRepository<'repo>: Repository<Section> + Send
    where
        Self: 'repo;

    // Each `'_` placeholder below is the per-call `&mut self` borrow that feeds
    // the corresponding repository GAT.
    fn project(&mut self) -> Self::ProjectRepository<'_>;
    fn card(&mut self) -> Self::CardRepository<'_>;
    fn section(&mut self) -> Self::SectionRepository<'_>;
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
    fn rollback(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

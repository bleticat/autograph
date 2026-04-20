use crate::card::entity::Card;
use crate::event::entity::Event;
use crate::project::entity::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use std::future::Future;

pub trait UnitOfWork: Send {
    type ProjectRepository<'a>: Repository<Project> + Send
    where
        Self: 'a;
    type CardRepository<'a>: Repository<Card> + Send
    where
        Self: 'a;
    type EventRepository<'a>: Repository<Event> + Send
    where
        Self: 'a;

    fn project(&mut self) -> Self::ProjectRepository<'_>;
    fn card(&mut self) -> Self::CardRepository<'_>;
    fn event(&mut self) -> Self::EventRepository<'_>;
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

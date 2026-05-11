use crate::card::queries::CardQueries;
use crate::project::queries::ProjectQueries;
use crate::section::queries::SectionQueries;
use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use std::future::Future;

pub trait Database: Sync {
    type Uow: UnitOfWork;
    type CardQueries<'db>: CardQueries + Send
    where
        Self: 'db;
    type ProjectQueries<'db>: ProjectQueries + Send
    where
        Self: 'db;
    type SectionQueries<'db>: SectionQueries + Send
    where
        Self: 'db;

    fn card(&self) -> Self::CardQueries<'_>;
    fn project(&self) -> Self::ProjectQueries<'_>;
    fn section(&self) -> Self::SectionQueries<'_>;
    fn begin<T: Send>(
        &self,
        f: impl AsyncFnOnce(&mut Self::Uow) -> Result<T, AppErr> + Send,
    ) -> impl Future<Output = Result<T, AppErr>>;
}

pub trait DatabaseBuilder: Sized {
    type Db: Database;

    fn open(path: &str) -> Self;
    fn migrate(self) -> Self;
    fn finish(self) -> impl Future<Output = Result<Self::Db, AppErr>> + Send;
}

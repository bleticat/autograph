use crate::card::adapters::seaorm::history_repository::SeaOrmCardHistoryRepository;
use crate::card::adapters::seaorm::repository::SeaOrmCardRepository;
use crate::project::adapters::seaorm::repository::SeaOrmProjectRepository;
use crate::section::adapters::seaorm::repository::SeaOrmSectionRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use sea_orm::DatabaseTransaction;

pub struct SeaOrmUnitOfWork {
    tx: DatabaseTransaction,
}

impl SeaOrmUnitOfWork {
    pub(super) fn new(tx: DatabaseTransaction) -> Self {
        Self { tx }
    }
}

impl UnitOfWork for SeaOrmUnitOfWork {
    type ProjectRepository<'a>
        = SeaOrmProjectRepository<'a>
    where
        Self: 'a;
    type CardRepository<'a>
        = SeaOrmCardRepository<'a>
    where
        Self: 'a;
    type CardHistoryRepository<'a>
        = SeaOrmCardHistoryRepository<'a>
    where
        Self: 'a;
    type SectionRepository<'a>
        = SeaOrmSectionRepository<'a>
    where
        Self: 'a;

    fn project(&mut self) -> SeaOrmProjectRepository<'_> {
        SeaOrmProjectRepository::new(&self.tx)
    }

    fn card(&mut self) -> SeaOrmCardRepository<'_> {
        SeaOrmCardRepository::new(&self.tx)
    }

    fn card_history(&mut self) -> SeaOrmCardHistoryRepository<'_> {
        SeaOrmCardHistoryRepository::new(&self.tx)
    }

    fn section(&mut self) -> SeaOrmSectionRepository<'_> {
        SeaOrmSectionRepository::new(&self.tx)
    }

    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

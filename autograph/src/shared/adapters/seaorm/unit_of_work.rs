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
    // These lifetimes mirror the UnitOfWork GAT contract: every returned
    // repository borrows this unit of work's transaction.
    type ProjectRepository<'repo>
        = SeaOrmProjectRepository<'repo>
    where
        Self: 'repo;
    type CardRepository<'repo>
        = SeaOrmCardRepository<'repo>
    where
        Self: 'repo;
    type SectionRepository<'repo>
        = SeaOrmSectionRepository<'repo>
    where
        Self: 'repo;

    // The `'_` placeholders are the method-call borrows used to build
    // transaction-scoped repositories.
    fn project(&mut self) -> SeaOrmProjectRepository<'_> {
        SeaOrmProjectRepository::new(&self.tx)
    }

    fn card(&mut self) -> SeaOrmCardRepository<'_> {
        SeaOrmCardRepository::new(&self.tx)
    }

    fn section(&mut self) -> SeaOrmSectionRepository<'_> {
        SeaOrmSectionRepository::new(&self.tx)
    }

    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }

    async fn rollback(self) -> Result<(), AppErr> {
        self.tx.rollback().await?;
        Ok(())
    }
}

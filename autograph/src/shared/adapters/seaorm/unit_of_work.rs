use crate::card::adapters::seaorm::repository::SeaOrmCardRepository;
use crate::project::adapters::seaorm::repository::SeaOrmProjectRepository;
use crate::section::adapters::seaorm::repository::SeaOrmSectionRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use sea_orm::DatabaseTransaction;

pub struct SeaOrmUnitOfWork {
    tx: Option<DatabaseTransaction>,
}

impl SeaOrmUnitOfWork {
    pub(super) fn new(tx: DatabaseTransaction) -> Self {
        Self { tx: Some(tx) }
    }
}

impl Drop for SeaOrmUnitOfWork {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            // Safety net: if neither commit nor rollback was called explicitly,
            // spawn a task to roll back the transaction. Note that if no tokio
            // runtime is active (e.g., during shutdown), this spawn will panic.
            // The `begin` helper always calls rollback explicitly on error, so
            // this path is only reached in unexpected drop scenarios.
            tokio::spawn(async move {
                let _ = tx.rollback().await;
            });
        }
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
    type SectionRepository<'a>
        = SeaOrmSectionRepository<'a>
    where
        Self: 'a;

    fn project(&mut self) -> SeaOrmProjectRepository<'_> {
        SeaOrmProjectRepository::new(self.tx.as_ref().expect("transaction was already committed or rolled back"))
    }

    fn card(&mut self) -> SeaOrmCardRepository<'_> {
        SeaOrmCardRepository::new(self.tx.as_ref().expect("transaction was already committed or rolled back"))
    }

    fn section(&mut self) -> SeaOrmSectionRepository<'_> {
        SeaOrmSectionRepository::new(self.tx.as_ref().expect("transaction was already committed or rolled back"))
    }

    async fn commit(mut self) -> Result<(), AppErr> {
        if let Some(tx) = self.tx.take() {
            tx.commit().await?;
        }
        Ok(())
    }

    async fn rollback(mut self) -> Result<(), AppErr> {
        if let Some(tx) = self.tx.take() {
            tx.rollback().await?;
        }
        Ok(())
    }
}

use crate::card::adapters::sqlx::repository::SqlxCardRepository;
use crate::project::adapters::sqlx::repository::SqlxProjectRepository;
use crate::section::adapters::sqlx::repository::SqlxSectionRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;

pub struct SqlxUnitOfWork {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl SqlxUnitOfWork {
    pub(super) fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl UnitOfWork for SqlxUnitOfWork {
    type ProjectRepository<'a>
        = SqlxProjectRepository<'a>
    where
        Self: 'a;
    type CardRepository<'a>
        = SqlxCardRepository<'a>
    where
        Self: 'a;
    type SectionRepository<'a>
        = SqlxSectionRepository<'a>
    where
        Self: 'a;

    fn project(&mut self) -> SqlxProjectRepository<'_> {
        SqlxProjectRepository::new(&mut self.tx)
    }

    fn card(&mut self) -> SqlxCardRepository<'_> {
        SqlxCardRepository::new(&mut self.tx)
    }

    fn section(&mut self) -> SqlxSectionRepository<'_> {
        SqlxSectionRepository::new(&mut self.tx)
    }

    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

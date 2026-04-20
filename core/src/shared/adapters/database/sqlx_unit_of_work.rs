use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;

pub struct SqlxUnitOfWork {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl SqlxUnitOfWork {
    pub(super) fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }

    pub(crate) fn tx(&mut self) -> &mut sqlx::Transaction<'static, sqlx::Sqlite> {
        &mut self.tx
    }
}

impl UnitOfWork for SqlxUnitOfWork {
    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

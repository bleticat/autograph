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
    type Tx = sqlx::Transaction<'static, sqlx::Sqlite>;

    fn tx(&mut self) -> &mut Self::Tx {
        &mut self.tx
    }

    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

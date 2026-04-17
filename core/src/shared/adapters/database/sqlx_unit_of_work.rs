use crate::projects::adapters::sqlx_project_repo::SqliteProjectRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::adapters::sqlx_task_repo::SqliteTodoRepository;

pub struct SqlxUnitOfWork {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl SqlxUnitOfWork {
    pub(super) fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl UnitOfWork for SqlxUnitOfWork {
    type ProjectRepo<'a> = SqliteProjectRepository<'a> where Self: 'a;
    type TaskRepo<'a> = SqliteTodoRepository<'a> where Self: 'a;

    fn projects(&mut self) -> SqliteProjectRepository<'_> {
        SqliteProjectRepository::new(&mut self.tx)
    }

    fn tasks(&mut self) -> SqliteTodoRepository<'_> {
        SqliteTodoRepository::new(&mut self.tx)
    }

    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

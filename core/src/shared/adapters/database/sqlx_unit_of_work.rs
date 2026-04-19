use crate::projects::adapters::sqlx_project_repo::SqlxProjectRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::adapters::sqlx_task_repo::SqlxTodoRepository;

pub struct SqlxUnitOfWork {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl SqlxUnitOfWork {
    pub(super) fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

impl UnitOfWork for SqlxUnitOfWork {
    type ProjectRepo<'a> = SqlxProjectRepository<'a> where Self: 'a;
    type TaskRepo<'a> = SqlxTodoRepository<'a> where Self: 'a;

    fn projects(&mut self) -> SqlxProjectRepository<'_> {
        SqlxProjectRepository::new(&mut self.tx)
    }

    fn tasks(&mut self) -> SqlxTodoRepository<'_> {
        SqlxTodoRepository::new(&mut self.tx)
    }

    async fn commit(self) -> Result<(), AppErr> {
        self.tx.commit().await?;
        Ok(())
    }
}

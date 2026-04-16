use crate::projects::adapters::sqlx_project_repo::SqliteProjectRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Database;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::tasks::adapters::sqlx_task_repo::SqliteTodoRepository;
use sqlx::sqlite::SqlitePoolOptions;
use std::str::FromStr;

pub type SqlxConnection = sqlx::SqlitePool;

pub struct SqlxUnitOfWork {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
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

pub struct SqlxDatabase {
    pool: SqlxConnection,
}

impl SqlxDatabase {
    pub async fn migrate(&self) -> Result<(), AppErr> {
        sqlx::migrate!("./src/shared/adapters/database/migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}

impl Database for SqlxDatabase {
    type Conn = SqlxConnection;
    type Uow = SqlxUnitOfWork;

    async fn open(path: &str) -> Result<Self, AppErr> {
        let path = path.to_owned();
        let is_memory = path == ":memory:";
        let conn_str = if is_memory {
            "sqlite::memory:".to_owned()
        } else {
            format!("sqlite://{path}")
        };

        let mut conn_options = sqlx::sqlite::SqliteConnectOptions::from_str(&conn_str)?
            .foreign_keys(true)
            .create_if_missing(!is_memory);

        if !is_memory {
            conn_options = conn_options.journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
        }

        let max_conns = if is_memory { 1 } else { 4 };
        let pool = SqlitePoolOptions::new()
            .max_connections(max_conns)
            .connect_with(conn_options)
            .await?;

        Ok(Self { pool })
    }

    fn conn(&self) -> SqlxConnection {
        self.pool.clone()
    }

    async fn transaction<'a, T: Send + 'a>(
        &'a self,
        f: impl AsyncFnOnce(&mut SqlxUnitOfWork) -> Result<T, AppErr> + Send + 'a,
    ) -> Result<T, AppErr> {
        let mut uow = self.pool.begin().await.map(|tx| SqlxUnitOfWork { tx })?;
        let val = f(&mut uow).await?;
        uow.commit().await?;
        Ok(val)
    }
}

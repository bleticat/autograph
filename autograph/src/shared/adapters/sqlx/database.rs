use super::unit_of_work::SqlxUnitOfWork;
use crate::shared::database::Database;
use crate::shared::error::AppErr;
use crate::shared::unit_of_work::UnitOfWork;
use sqlx::sqlite::SqlitePoolOptions;
use std::str::FromStr;

pub type SqlxConnection = sqlx::SqlitePool;

pub struct SqlxDatabase {
    pool: SqlxConnection,
}

impl SqlxDatabase {
    pub async fn migrate(&self) -> Result<(), AppErr> {
        sqlx::migrate!("./src/shared/adapters/sqlx/migrations")
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

    async fn begin<'a, T: Send + 'a>(
        &'a self,
        f: impl AsyncFnOnce(&mut SqlxUnitOfWork) -> Result<T, AppErr> + Send + 'a,
    ) -> Result<T, AppErr> {
        let mut uow = self.pool.begin().await.map(SqlxUnitOfWork::new)?;
        let val = f(&mut uow).await?;
        uow.commit().await?;
        Ok(val)
    }
}

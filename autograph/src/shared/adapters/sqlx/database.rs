use super::unit_of_work::SqlxUnitOfWork;
use crate::shared::error::AppErr;
use crate::shared::ports::database::{Database, DatabaseBuilder};
use crate::shared::ports::unit_of_work::UnitOfWork;
use sqlx::sqlite::SqlitePoolOptions;
use std::str::FromStr;

pub type SqlxConnection = sqlx::SqlitePool;

pub struct SqlxDatabase {
    pool: SqlxConnection,
}

pub struct SqlxDatabaseBuilder {
    path: String,
    run_migrations: bool,
}

impl DatabaseBuilder for SqlxDatabaseBuilder {
    type Db = SqlxDatabase;

    fn open(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            run_migrations: false,
        }
    }

    fn migrate(self) -> Self {
        Self {
            run_migrations: true,
            ..self
        }
    }

    async fn finish(self) -> Result<SqlxDatabase, AppErr> {
        let is_memory = self.path == ":memory:";
        let conn_str = if is_memory {
            "sqlite::memory:".to_owned()
        } else {
            format!("sqlite://{}", self.path)
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

        if self.run_migrations {
            sqlx::migrate!("./src/shared/adapters/sqlx/migrations")
                .run(&pool)
                .await?;
        }

        Ok(SqlxDatabase { pool })
    }
}

impl Database for SqlxDatabase {
    type Conn = SqlxConnection;
    type Uow = SqlxUnitOfWork;

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

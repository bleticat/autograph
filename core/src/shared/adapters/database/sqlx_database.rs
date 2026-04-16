use crate::shared::error::AppErr;
use crate::shared::ports::database::Database;
use sqlx::sqlite::SqlitePoolOptions;
use std::str::FromStr;
use tokio::sync::Mutex;

pub type SqlxConn = sqlx::SqlitePool;
pub type SqlxTx = Mutex<sqlx::Transaction<'static, sqlx::Sqlite>>;

pub struct SqlxDatabase {
    pool: SqlxConn,
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
    type Conn<'a> = SqlxConn;
    type Tx<'a> = SqlxTx;

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

    fn conn(&self) -> SqlxConn {
        self.pool.clone()
    }

    async fn transaction<'a, T: 'a>(
        &'a self,
        f: impl AsyncFnOnce(&SqlxTx) -> Result<T, AppErr> + 'a,
    ) -> Result<T, AppErr> {
        let tx = Mutex::new(self.pool.begin().await?);
        match f(&tx).await {
            Ok(val) => {
                tx.into_inner().commit().await?;
                Ok(val)
            }
            Err(e) => {
                let _ = tx.into_inner().rollback().await;
                Err(e)
            }
        }
    }
}

use crate::shared::error::AppErr;
use crate::shared::ports::database::Database;
use sqlx::sqlite::SqlitePoolOptions;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
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
    type Conn = SqlxConn;
    type Tx = SqlxTx;

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

    async fn transaction<'a, T, F, Fut>(&'a self, f: F) -> Result<T, AppErr>
    where
        T: Send + 'a,
        F: FnOnce(Arc<SqlxTx>) -> Fut + Send + 'a,
        Fut: Future<Output = Result<T, AppErr>> + Send + 'a,
    {
        let tx = Arc::new(Mutex::new(self.pool.begin().await?));
        let val = f(Arc::clone(&tx)).await?;
        let tx = Arc::try_unwrap(tx).ok()
            .expect("transaction Arc should have no other owners");
        tx.into_inner().commit().await?;
        Ok(val)
    }
}

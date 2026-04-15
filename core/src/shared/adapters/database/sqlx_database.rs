use crate::shared::error::AppErr;
use crate::shared::ports::database::{Connection, Database, Transaction};
use sqlx::sqlite::SqlitePoolOptions;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct SqlxConnection(sqlx::SqlitePool);

impl SqlxConnection {
    pub(crate) fn raw(&self) -> sqlx::SqlitePool {
        self.0.clone()
    }
}

impl Connection for SqlxConnection {}

#[derive(Clone)]
pub struct SqlxTransaction(Arc<Mutex<Option<sqlx::Transaction<'static, sqlx::Sqlite>>>>);

impl SqlxTransaction {
    fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self(Arc::new(Mutex::new(Some(tx))))
    }

    pub(crate) fn raw(&self) -> Arc<Mutex<Option<sqlx::Transaction<'static, sqlx::Sqlite>>>> {
        self.0.clone()
    }

    async fn commit(&self) -> Result<(), AppErr> {
        let mut tx = self.0.lock().await;
        if let Some(tx) = tx.take() {
            tx.commit().await?;
        }
        Ok(())
    }

    async fn rollback(&self) -> Result<(), AppErr> {
        let mut tx = self.0.lock().await;
        if let Some(tx) = tx.take() {
            tx.rollback().await?;
        }
        Ok(())
    }
}

impl Transaction for SqlxTransaction {
    type Conn = SqlxConnection;
}

pub struct SqlxDatabase {
    pool: sqlx::SqlitePool,
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
    type Conn<'a> = SqlxConnection;
    type Tx<'a> = SqlxTransaction;

    fn open(path: &str) -> impl Future<Output = Result<Self, AppErr>> {
        let path = path.to_owned();
        async move {
            let is_memory = path == ":memory:";
            let conn_str = if is_memory {
                "sqlite::memory:".to_owned()
            } else {
                format!("sqlite://{path}")
            };

            let conn_options = sqlx::sqlite::SqliteConnectOptions::from_str(&conn_str)?
                .foreign_keys(true)
                .create_if_missing(!is_memory);
            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .connect_with(conn_options)
                .await?;

            Ok(Self { pool })
        }
    }

    fn conn(&self) -> SqlxConnection {
        SqlxConnection(self.pool.clone())
    }

    fn transaction<T, F>(
        &self,
        f: impl FnOnce(SqlxTransaction) -> F,
    ) -> impl Future<Output = Result<T, AppErr>>
    where
        F: Future<Output = Result<T, AppErr>>,
    {
        async move {
            let tx = SqlxTransaction::new(self.pool.begin().await?);
            match f(tx.clone()).await {
                Ok(val) => {
                    tx.commit().await?;
                    Ok(val)
                }
                Err(e) => {
                    let _ = tx.rollback().await;
                    Err(e)
                }
            }
        }
    }
}

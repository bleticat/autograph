use crate::shared::error::AppErr;
use std::future::Future;

pub trait Connection {}

pub trait Transaction {
    type Conn: Connection;
}

#[allow(async_fn_in_trait)]
pub trait Database {
    type Conn<'a>: Connection
    where
        Self: 'a;
    type Tx<'a>: Transaction
    where
        Self: 'a;

    async fn open(path: &str) -> Result<Self, AppErr>
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn<'_>;
    async fn transaction<T, F>(&self, f: impl FnOnce(Self::Tx<'_>) -> F) -> Result<T, AppErr>
    where
        F: Future<Output = Result<T, AppErr>>;
}

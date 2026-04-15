use crate::shared::error::AppErr;
use std::future::Future;

pub trait Connection {}

pub trait Transaction {
    type Conn: Connection;
}

pub trait Database {
    type Conn<'a>: Connection
    where
        Self: 'a;
    type Tx<'a>: Transaction
    where
        Self: 'a;

    fn open(path: &str) -> impl Future<Output = Result<Self, AppErr>>
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn<'_>;
    fn transaction<T, F>(
        &self,
        f: impl FnOnce(Self::Tx<'_>) -> F,
    ) -> impl Future<Output = Result<T, AppErr>>
    where
        F: Future<Output = Result<T, AppErr>>;
}

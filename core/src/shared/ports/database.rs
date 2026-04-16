use crate::shared::error::AppErr;
use std::future::Future;

pub trait Connection {}

pub trait Transaction {}

pub trait Database {
    type Conn<'a>: Connection
    where
        Self: 'a;
    type Tx<'a>: Transaction
    where
        Self: 'a;

    fn open(path: &str) -> impl Future<Output = Result<Self, AppErr>> + Send + '_
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn<'_>;
    fn transaction<'a, T: 'a>(
        &'a self,
        f: impl AsyncFnOnce(&Self::Tx<'_>) -> Result<T, AppErr> + 'a,
    ) -> impl Future<Output = Result<T, AppErr>> + 'a;
}

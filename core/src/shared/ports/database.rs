use crate::shared::error::AppErr;
use std::future::Future;
use std::sync::Arc;

pub trait Database {
    type Conn;
    type Tx: Send + Sync + 'static;

    fn open(path: &str) -> impl Future<Output = Result<Self, AppErr>> + Send + '_
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn;
    fn transaction<'a, T, F, Fut>(&'a self, f: F) -> impl Future<Output = Result<T, AppErr>> + Send + 'a
    where
        T: Send + 'a,
        F: FnOnce(Arc<Self::Tx>) -> Fut + Send + 'a,
        Fut: Future<Output = Result<T, AppErr>> + Send + 'a;
}

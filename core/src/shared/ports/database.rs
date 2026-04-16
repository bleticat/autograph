use crate::shared::error::AppErr;
use std::future::Future;

pub trait Database {
    type Conn<'a>
    where
        Self: 'a;
    type Tx<'a>
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

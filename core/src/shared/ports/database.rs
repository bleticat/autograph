use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use std::future::Future;

pub trait Database: Sync {
    type Conn;
    type Uow: UnitOfWork;

    fn open(path: &str) -> impl Future<Output = Result<Self, AppErr>> + Send + '_
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn;
    fn transaction<'a, T: Send + 'a>(
        &'a self,
        f: impl AsyncFnOnce(&mut Self::Uow) -> Result<T, AppErr> + Send + 'a,
    ) -> impl Future<Output = Result<T, AppErr>> + 'a;
}

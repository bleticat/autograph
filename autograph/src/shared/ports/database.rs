use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use std::future::Future;

pub trait Database: Sync {
    type Conn;
    type Uow: UnitOfWork;

    fn conn(&self) -> Self::Conn;
    fn begin<'a, T: Send + 'a>(
        &'a self,
        f: impl AsyncFnOnce(&mut Self::Uow) -> Result<T, AppErr> + Send + 'a,
    ) -> impl Future<Output = Result<T, AppErr>> + 'a;
}

pub trait DatabaseBuilder: Sized {
    type Db: Database;

    fn open(path: &str) -> Self;
    fn migrate(self) -> Self;
    fn finish(self) -> impl Future<Output = Result<Self::Db, AppErr>> + Send;
}

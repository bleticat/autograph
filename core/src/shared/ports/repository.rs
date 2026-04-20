use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use std::future::Future;
use uuid::Uuid;

pub trait Repository: Sized {
    type Tx;

    fn get<U>(uow: &mut U, id: Uuid) -> impl Future<Output = Result<Option<Self>, AppErr>> + Send
    where
        U: UnitOfWork<Tx = Self::Tx>;

    fn save<U>(self, uow: &mut U) -> impl Future<Output = Result<Self, AppErr>> + Send
    where
        U: UnitOfWork<Tx = Self::Tx>;

    fn delete<U>(uow: &mut U, id: Uuid) -> impl Future<Output = Result<(), AppErr>> + Send
    where
        U: UnitOfWork<Tx = Self::Tx>;
}

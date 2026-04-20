use crate::shared::error::AppErr;
use crate::shared::ports::unit_of_work::UnitOfWork;
use std::future::Future;
use uuid::Uuid;

/// Entity-level repository operations executed within a unit of work.
pub trait Repository: Sized {
    /// Transaction type required by this entity for persistence operations.
    type Tx;

    /// Load an entity by id using the given unit of work.
    fn get<U>(uow: &mut U, id: Uuid) -> impl Future<Output = Result<Option<Self>, AppErr>> + Send
    where
        U: UnitOfWork<Tx = Self::Tx>;

    /// Persist this entity using the given unit of work.
    fn save<U>(self, uow: &mut U) -> impl Future<Output = Result<Self, AppErr>> + Send
    where
        U: UnitOfWork<Tx = Self::Tx>;

    /// Delete an entity by id using the given unit of work.
    fn delete<U>(uow: &mut U, id: Uuid) -> impl Future<Output = Result<(), AppErr>> + Send
    where
        U: UnitOfWork<Tx = Self::Tx>;
}

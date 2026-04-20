use crate::shared::error::AppErr;
use std::future::Future;

pub trait UnitOfWork: Send {
    /// Underlying transaction type used by this unit of work implementation.
    type Tx;
    fn tx(&mut self) -> &mut Self::Tx;
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

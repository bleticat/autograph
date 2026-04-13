use crate::shared::error::AppErr;

/// A read-only handle to the underlying database connection.
/// Operations that only read data (queries) run through a `Connection`.
pub trait Connection {}

/// A write-capable handle associated with an open transaction.
/// All mutations should run through a `Transaction` so that they can be
/// committed or rolled back atomically.
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

    fn open(path: &str) -> Result<Self, AppErr>
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn<'_>;
    fn transaction<T>(
        &self,
        f: impl FnOnce(Self::Tx<'_>) -> Result<T, AppErr>,
    ) -> Result<T, AppErr>;
}

use crate::shared::error::AppErr;

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

    fn open(path: &str) -> Result<Self, AppErr>
    where
        Self: Sized;
    fn conn(&self) -> Self::Conn<'_>;
    fn transaction<T>(
        &self,
        f: impl FnOnce(Self::Tx<'_>) -> Result<T, AppErr>,
    ) -> Result<T, AppErr>;
}

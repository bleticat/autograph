use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;

pub trait ProjectRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn add(&self, title: &str) -> Result<i64, AppErr>;
}

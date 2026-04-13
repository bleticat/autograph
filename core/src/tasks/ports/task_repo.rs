use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;

pub trait TodoRepository: From<Self::Tx> {
    type Tx: Transaction;
    fn add(&self, title: &str) -> Result<i64, AppErr>;
    fn toggle(&self, id: i64) -> Result<(), AppErr>;
    fn delete(&self, id: i64) -> Result<(), AppErr>;
}

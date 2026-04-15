use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;

pub trait Repository<Entity, Id>: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: Id) -> Result<Option<Entity>, AppErr>;
    fn save(&self, entity: &Entity) -> Result<Id, AppErr>;
    fn delete(&self, id: Id) -> Result<(), AppErr>;
}

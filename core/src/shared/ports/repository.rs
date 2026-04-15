use crate::shared::error::AppErr;
use crate::shared::ports::database::Transaction;
use uuid::Uuid;

pub trait Repository<Entity>: From<Self::Tx> {
    type Tx: Transaction;
    fn get(&self, id: Uuid) -> Result<Option<Entity>, AppErr>;
    fn save(&self, entity: &Entity) -> Result<Uuid, AppErr>;
    fn delete(&self, id: Uuid) -> Result<(), AppErr>;
}

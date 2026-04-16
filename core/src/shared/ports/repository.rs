use crate::shared::error::AppErr;
use std::future::Future;
use uuid::Uuid;

pub trait Repository<Entity> {
    fn get(&mut self, id: Uuid)
    -> impl Future<Output = Result<Option<Entity>, AppErr>> + Send + '_;
    fn save(&mut self, entity: Entity) -> impl Future<Output = Result<Entity, AppErr>> + Send + '_;
    fn delete(&mut self, id: Uuid) -> impl Future<Output = Result<(), AppErr>> + Send + '_;
}

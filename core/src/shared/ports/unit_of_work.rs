use crate::events::Event;
use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use std::future::Future;

pub trait UnitOfWork: Send + Repository<Project> + Repository<Todo> + Repository<Event> {
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

use crate::projects::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::tasks::Todo;
use std::future::Future;

pub trait UnitOfWork: Send {
    type ProjectRepo<'a>: Repository<Project> + Send
    where
        Self: 'a;
    type TaskRepo<'a>: Repository<Todo> + Send
    where
        Self: 'a;

    fn projects(&mut self) -> Self::ProjectRepo<'_>;
    fn tasks(&mut self) -> Self::TaskRepo<'_>;
    fn commit(self) -> impl Future<Output = Result<(), AppErr>> + Send;
}

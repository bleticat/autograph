use crate::project::entity::Project;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use uuid::Uuid;

/// Command facade scoped to a borrowed unit of work.
///
/// `'uow` is explicit because the facade stores `&mut U`; this prevents command
/// operations from outliving the transaction they mutate.
pub struct ProjectCommands<'uow, U: UnitOfWork> {
    uow: &'uow mut U,
}

// The impl names `'uow` so `new` can return a facade tied to the incoming
// mutable unit-of-work borrow.
impl<'uow, U: UnitOfWork> ProjectCommands<'uow, U> {
    pub fn new(uow: &'uow mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Project, AppErr> {
        self.uow
            .project()
            .save(Project {
                id: Uuid::nil(),
                title: title.to_owned(),
            })
            .await
    }
}

use crate::project::entity::Project;
use crate::shared::adapters::seaorm::models::project as project_model;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use uuid::Uuid;

/// Repository scoped to an active SeaORM transaction.
///
/// `'tx` is explicit because the repository stores `&DatabaseTransaction`; it
/// must not outlive the transaction used by its queries.
pub struct SeaOrmProjectRepository<'tx> {
    tx: &'tx DatabaseTransaction,
}

// The impl names `'tx` so `new` can return a repository tied to the incoming
// transaction borrow.
impl<'tx> SeaOrmProjectRepository<'tx> {
    pub fn new(tx: &'tx DatabaseTransaction) -> Self {
        Self { tx }
    }
}

// `'_` makes the trait impl cover repositories tied to any transaction borrow.
impl Repository<Project> for SeaOrmProjectRepository<'_> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Project>, AppErr> {
        Ok(project_model::Entity::find_by_id(id)
            .one(self.tx)
            .await?
            .map(to_project))
    }

    async fn save(&mut self, project: Project) -> Result<Project, AppErr> {
        if project.id.is_nil() {
            let saved = project_model::ActiveModel {
                id: Set(Uuid::new_v4()),
                title: Set(project.title),
            }
            .insert(self.tx)
            .await?;

            Ok(to_project(saved))
        } else {
            let saved = project_model::ActiveModel {
                id: Set(project.id),
                title: Set(project.title),
            }
            .update(self.tx)
            .await?;

            Ok(to_project(saved))
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        project_model::Entity::delete_by_id(id)
            .exec(self.tx)
            .await?;
        Ok(())
    }
}

fn to_project(model: project_model::Model) -> Project {
    Project {
        id: model.id,
        title: model.title,
    }
}

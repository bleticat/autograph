use crate::section::entity::Section;
use crate::shared::adapters::seaorm::models::section as section_model;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use sea_orm::{ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};
use uuid::Uuid;

// 'a ties the repository to the borrowed DatabaseTransaction, ensuring the
// repository cannot outlive the transaction it uses to execute queries.
pub struct SeaOrmSectionRepository<'a> {
    tx: &'a DatabaseTransaction,
}

impl<'a> SeaOrmSectionRepository<'a> {
    pub fn new(tx: &'a DatabaseTransaction) -> Self {
        Self { tx }
    }
}

impl<'a> Repository<Section> for SeaOrmSectionRepository<'a> {
    async fn get(&mut self, id: Uuid) -> Result<Option<Section>, AppErr> {
        Ok(section_model::Entity::find_by_id(id)
            .one(self.tx)
            .await?
            .map(to_section))
    }

    async fn save(&mut self, section: Section) -> Result<Section, AppErr> {
        if section.id.is_nil() {
            let saved = section_model::ActiveModel {
                id: Set(Uuid::new_v4()),
                title: Set(section.title),
                project_id: Set(section.project_id),
            }
            .insert(self.tx)
            .await?;

            Ok(to_section(saved))
        } else {
            let saved = section_model::ActiveModel {
                id: Set(section.id),
                title: Set(section.title),
                project_id: Set(section.project_id),
            }
            .update(self.tx)
            .await?;

            Ok(to_section(saved))
        }
    }

    async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        section_model::Entity::delete_by_id(id)
            .exec(self.tx)
            .await?;
        Ok(())
    }
}

fn to_section(model: section_model::Model) -> Section {
    Section {
        id: model.id,
        title: model.title,
        project_id: model.project_id,
    }
}

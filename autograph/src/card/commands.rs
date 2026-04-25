use crate::card::entity::Card;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Command facade scoped to a borrowed unit of work.
///
/// `'uow` is explicit because the facade stores `&mut U`; this prevents command
/// operations from outliving the transaction they mutate.
pub struct CardCommands<'uow, U: UnitOfWork> {
    uow: &'uow mut U,
}

// The impl names `'uow` so `new` can return a facade tied to the incoming
// mutable unit-of-work borrow.
impl<'uow, U: UnitOfWork> CardCommands<'uow, U> {
    pub fn new(uow: &'uow mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Card, AppErr> {
        self.add_with_assignment(title, None, None).await
    }

    pub async fn add_with_project(
        &mut self,
        title: &str,
        project_id: Uuid,
    ) -> Result<Card, AppErr> {
        self.add_with_assignment(title, Some(project_id), None)
            .await
    }

    pub async fn add_with_section(
        &mut self,
        title: &str,
        project_id: Option<Uuid>,
        section_id: Uuid,
    ) -> Result<Card, AppErr> {
        self.add_with_assignment(title, project_id, Some(section_id))
            .await
    }

    pub async fn add_with_assignment(
        &mut self,
        title: &str,
        project_id: Option<Uuid>,
        section_id: Option<Uuid>,
    ) -> Result<Card, AppErr> {
        let (project_id, section_id) = self.resolve_assignment(project_id, section_id).await?;
        self.uow
            .card()
            .save(Card {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id,
                section_id,
            })
            .await
    }

    pub async fn edit(
        &mut self,
        id: Uuid,
        title: &str,
        description: &str,
        deadline: Option<DateTime<Utc>>,
        project_id: Option<Uuid>,
        section_id: Option<Uuid>,
    ) -> Result<(), AppErr> {
        let card = self.uow.card().get(id).await?;
        if let Some(mut card) = card {
            let (project_id, section_id) = self.resolve_assignment(project_id, section_id).await?;
            card.title = title.to_owned();
            card.description = description.to_owned();
            card.deadline = deadline;
            card.project_id = project_id;
            card.section_id = section_id;
            self.uow.card().save(card).await?;
        }
        Ok(())
    }

    pub async fn toggle(&mut self, id: Uuid) -> Result<(), AppErr> {
        let card = self.uow.card().get(id).await?;
        if let Some(mut card) = card {
            card.completed = !card.completed;
            self.uow.card().save(card).await?;
        }
        Ok(())
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        self.uow.card().delete(id).await
    }

    async fn resolve_assignment(
        &mut self,
        project_id: Option<Uuid>,
        section_id: Option<Uuid>,
    ) -> Result<(Option<Uuid>, Option<Uuid>), AppErr> {
        let Some(section_id) = section_id else {
            return Ok((project_id, None));
        };

        let section = self
            .uow
            .section()
            .get(section_id)
            .await?
            .ok_or_else(|| AppErr::Validation("Section does not exist".to_owned()))?;

        if let Some(project_id) = project_id
            && project_id != section.project_id
        {
            return Err(AppErr::Validation(
                "Section must belong to the selected project".to_owned(),
            ));
        }

        Ok((Some(section.project_id), Some(section_id)))
    }
}

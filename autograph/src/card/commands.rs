use crate::card::entity::{Card, CardHistory};
use crate::card::history_repository::CardHistoryRepository;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub async fn append_history_and_rebuild<U: UnitOfWork>(
    uow: &mut U,
    id: Uuid,
    items: Vec<CardHistory>,
) -> Result<Card, AppErr> {
    uow.card_history().append_history(id, items).await?;
    let history = uow.card_history().get_history(id).await?;
    let card = Card::apply(history)?;
    uow.card().save(card).await
}

pub struct CardCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> CardCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
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
        let id = Uuid::new_v4();
        let mut history = vec![CardHistory::CreateCard {
            id,
            title: title.to_owned(),
        }];

        if project_id.is_some() {
            history.push(CardHistory::BindProject { project_id });
        }

        if section_id.is_some() {
            history.push(CardHistory::BindSection { section_id });
        }

        append_history_and_rebuild(self.uow, id, history).await
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
        if let Some(card) = card {
            if card.deleted {
                return Ok(());
            }

            let (project_id, section_id) = self.resolve_assignment(project_id, section_id).await?;
            let mut history = Vec::new();

            if card.title != title {
                history.push(CardHistory::ChangeTitle {
                    title: title.to_owned(),
                });
            }

            if card.description != description {
                history.push(CardHistory::ChangeDescription {
                    description: description.to_owned(),
                });
            }

            if card.deadline != deadline {
                history.push(CardHistory::ChangeDeadline { deadline });
            }

            if card.section_id != section_id {
                history.push(CardHistory::BindSection { section_id });
            }

            if card.project_id != project_id {
                history.push(CardHistory::BindProject { project_id });
            }

            if !history.is_empty() {
                append_history_and_rebuild(self.uow, id, history).await?;
            }
        }
        Ok(())
    }

    pub async fn delete(&mut self, id: Uuid) -> Result<(), AppErr> {
        let card = self.uow.card().get(id).await?;
        if let Some(card) = card {
            if card.deleted {
                return Ok(());
            }

            append_history_and_rebuild(self.uow, id, vec![CardHistory::DeleteCard]).await?;
        }

        Ok(())
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

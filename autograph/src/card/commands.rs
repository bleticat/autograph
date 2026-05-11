use crate::card::entity::Card;
use crate::shared::error::AppErr;
use crate::shared::ports::database::Database;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Command facade that holds a reference to the database and manages its own
/// transactions. Each command method opens a transaction, performs its work,
/// and commits (or rolls back on error).
pub struct CardCommands<'db, D: Database> {
    db: &'db D,
}

impl<'db, D: Database> CardCommands<'db, D> {
    pub fn new(db: &'db D) -> Self {
        Self { db }
    }

    pub async fn add(&self, title: &str) -> Result<Card, AppErr> {
        self.add_with_assignment(title, None, None).await
    }

    pub async fn add_with_project(
        &self,
        title: &str,
        project_id: Uuid,
    ) -> Result<Card, AppErr> {
        self.add_with_assignment(title, Some(project_id), None)
            .await
    }

    pub async fn add_with_section(
        &self,
        title: &str,
        project_id: Option<Uuid>,
        section_id: Uuid,
    ) -> Result<Card, AppErr> {
        self.add_with_assignment(title, project_id, Some(section_id))
            .await
    }

    pub async fn add_with_assignment(
        &self,
        title: &str,
        project_id: Option<Uuid>,
        section_id: Option<Uuid>,
    ) -> Result<Card, AppErr> {
        let title = title.to_owned();
        self.db
            .begin(async move |uow| {
                let (project_id, section_id) =
                    resolve_assignment(uow, project_id, section_id).await?;
                uow.card()
                    .save(Card {
                        id: Uuid::nil(),
                        title,
                        description: String::new(),
                        deadline: None,
                        completed: false,
                        project_id,
                        section_id,
                    })
                    .await
            })
            .await
    }

    pub async fn edit(
        &self,
        id: Uuid,
        title: &str,
        description: &str,
        deadline: Option<DateTime<Utc>>,
        project_id: Option<Uuid>,
        section_id: Option<Uuid>,
    ) -> Result<(), AppErr> {
        let title = title.to_owned();
        let description = description.to_owned();
        self.db
            .begin(async move |uow| {
                let card = uow.card().get(id).await?;
                if let Some(mut card) = card {
                    let (project_id, section_id) =
                        resolve_assignment(uow, project_id, section_id).await?;
                    card.title = title;
                    card.description = description;
                    card.deadline = deadline;
                    card.project_id = project_id;
                    card.section_id = section_id;
                    uow.card().save(card).await?;
                }
                Ok(())
            })
            .await
    }

    pub async fn toggle(&self, id: Uuid) -> Result<(), AppErr> {
        self.db
            .begin(async move |uow| {
                let card = uow.card().get(id).await?;
                if let Some(mut card) = card {
                    card.completed = !card.completed;
                    uow.card().save(card).await?;
                }
                Ok(())
            })
            .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppErr> {
        self.db
            .begin(async move |uow| uow.card().delete(id).await)
            .await
    }
}

async fn resolve_assignment<U: UnitOfWork>(
    uow: &mut U,
    project_id: Option<Uuid>,
    section_id: Option<Uuid>,
) -> Result<(Option<Uuid>, Option<Uuid>), AppErr> {
    let Some(section_id) = section_id else {
        return Ok((project_id, None));
    };

    let section = uow
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

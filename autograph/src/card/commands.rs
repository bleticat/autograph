use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use crate::card::Card;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct CardCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> CardCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(&mut self, title: &str) -> Result<Card, AppErr> {
        self.uow
            .card()
            .save(Card {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: None,
            })
            .await
    }

    pub async fn add_with_project(
        &mut self,
        title: &str,
        project_id: Uuid,
    ) -> Result<Card, AppErr> {
        self.uow
            .card()
            .save(Card {
                id: Uuid::nil(),
                title: title.to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: Some(project_id),
            })
            .await
    }

    pub async fn edit(
        &mut self,
        id: Uuid,
        title: &str,
        description: &str,
        deadline: Option<OffsetDateTime>,
    ) -> Result<(), AppErr> {
        let card = self.uow.card().get(id).await?;
        if let Some(mut card) = card {
            card.title = title.to_owned();
            card.description = description.to_owned();
            card.deadline = deadline;
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
}

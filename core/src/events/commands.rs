use crate::events::Event;
use crate::shared::error::AppErr;
use crate::shared::ports::repository::Repository;
use crate::shared::ports::unit_of_work::UnitOfWork;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct EventCommands<'a, U: UnitOfWork> {
    uow: &'a mut U,
}

impl<'a, U: UnitOfWork> EventCommands<'a, U> {
    pub fn new(uow: &'a mut U) -> Self {
        Self { uow }
    }

    pub async fn add(
        &mut self,
        date: OffsetDateTime,
        title: &str,
        description: &str,
    ) -> Result<Event, AppErr> {
        <U as Repository<Event>>::save(
            self.uow,
            Event {
                id: Uuid::nil(),
                date,
                title: title.to_owned(),
                description: description.to_owned(),
                project_id: None,
            },
        )
        .await
    }

    pub async fn add_with_project(
        &mut self,
        date: OffsetDateTime,
        title: &str,
        description: &str,
        project_id: Uuid,
    ) -> Result<Event, AppErr> {
        <U as Repository<Event>>::save(
            self.uow,
            Event {
                id: Uuid::nil(),
                date,
                title: title.to_owned(),
                description: description.to_owned(),
                project_id: Some(project_id),
            },
        )
        .await
    }

    pub async fn edit(
        &mut self,
        id: Uuid,
        date: OffsetDateTime,
        title: &str,
        description: &str,
    ) -> Result<(), AppErr> {
        let event = <U as Repository<Event>>::get(self.uow, id).await?;
        if let Some(mut event) = event {
            event.date = date;
            event.title = title.to_owned();
            event.description = description.to_owned();
            <U as Repository<Event>>::save(self.uow, event).await?;
        }
        Ok(())
    }
}

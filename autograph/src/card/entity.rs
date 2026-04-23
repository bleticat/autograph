use crate::shared::error::AppErr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CardHistory {
    CreateCard { id: Uuid, title: String },
    BindProject { project_id: Option<Uuid> },
    BindSection { section_id: Option<Uuid> },
    ChangeDescription { description: String },
    ChangeDeadline { deadline: Option<DateTime<Utc>> },
    ChangeTitle { title: String },
    DeleteCard,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub deadline: Option<DateTime<Utc>>,
    pub deleted: bool,
    pub project_id: Option<Uuid>,
    pub section_id: Option<Uuid>,
    pub history: Vec<CardHistory>,
}

impl Card {
    pub fn apply(history: Vec<CardHistory>) -> Result<Self, AppErr> {
        let mut card = None;

        for item in &history {
            match item {
                CardHistory::CreateCard { id, title } => {
                    if card.is_some() {
                        return Err(AppErr::Validation(
                            "Card history contains more than one CreateCard event".to_owned(),
                        ));
                    }

                    card = Some(Self {
                        id: *id,
                        title: title.clone(),
                        description: String::new(),
                        deadline: None,
                        deleted: false,
                        project_id: None,
                        section_id: None,
                        history: Vec::new(),
                    });
                }
                CardHistory::BindProject { project_id } => {
                    let card = card.as_mut().ok_or_else(|| {
                        AppErr::Validation(
                            "Card history must start with a CreateCard event".to_owned(),
                        )
                    })?;
                    card.project_id = *project_id;
                    if project_id.is_none() {
                        card.section_id = None;
                    }
                }
                CardHistory::BindSection { section_id } => {
                    let card = card.as_mut().ok_or_else(|| {
                        AppErr::Validation(
                            "Card history must start with a CreateCard event".to_owned(),
                        )
                    })?;
                    card.section_id = *section_id;
                }
                CardHistory::ChangeDescription { description } => {
                    let card = card.as_mut().ok_or_else(|| {
                        AppErr::Validation(
                            "Card history must start with a CreateCard event".to_owned(),
                        )
                    })?;
                    card.description = description.clone();
                }
                CardHistory::ChangeDeadline { deadline } => {
                    let card = card.as_mut().ok_or_else(|| {
                        AppErr::Validation(
                            "Card history must start with a CreateCard event".to_owned(),
                        )
                    })?;
                    card.deadline = *deadline;
                }
                CardHistory::ChangeTitle { title } => {
                    let card = card.as_mut().ok_or_else(|| {
                        AppErr::Validation(
                            "Card history must start with a CreateCard event".to_owned(),
                        )
                    })?;
                    card.title = title.clone();
                }
                CardHistory::DeleteCard => {
                    let card = card.as_mut().ok_or_else(|| {
                        AppErr::Validation(
                            "Card history must start with a CreateCard event".to_owned(),
                        )
                    })?;
                    card.deleted = true;
                }
            }
        }

        let mut card = card.ok_or_else(|| {
            AppErr::Validation("Card history does not contain a CreateCard event".to_owned())
        })?;
        card.history = history;
        Ok(card)
    }
}

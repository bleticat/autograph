use crate::card::entity::Card;
use crate::section::entity::Section;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SectionWithCards {
    pub section: Section,
    pub cards: Vec<Card>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub project: Project,
    pub sections: Vec<SectionWithCards>,
    pub cards_without_section: Vec<Card>,
}

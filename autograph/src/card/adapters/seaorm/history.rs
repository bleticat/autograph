use crate::card::entity::{Card, CardHistory};
use crate::shared::adapters::seaorm::models::{
    card as card_model, card_history as card_history_model,
};
use crate::shared::error::AppErr;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) fn serialize_history(items: &[CardHistory]) -> Result<String, AppErr> {
    serde_json::to_string(items)
        .map_err(|err| AppErr::Parse(format!("Failed to serialize card history: {err}")))
}

fn deserialize_history(raw: &str) -> Result<Vec<CardHistory>, AppErr> {
    serde_json::from_str(raw)
        .map_err(|err| AppErr::Parse(format!("Failed to deserialize card history: {err}")))
}

pub(crate) async fn load_history<C: ConnectionTrait>(
    conn: &C,
    id: Uuid,
) -> Result<Vec<CardHistory>, AppErr> {
    let history = card_history_model::Entity::find_by_id(id).one(conn).await?;
    history
        .map(|model| deserialize_history(&model.items))
        .transpose()
        .map(|history| history.unwrap_or_default())
}

pub(crate) async fn load_history_map<C: ConnectionTrait>(
    conn: &C,
    ids: &[Uuid],
) -> Result<HashMap<Uuid, Vec<CardHistory>>, AppErr> {
    if ids.is_empty() {
        return Ok(HashMap::new());
    }

    let history_rows = card_history_model::Entity::find()
        .filter(card_history_model::Column::CardId.is_in(ids.iter().copied()))
        .all(conn)
        .await?;

    let mut history_map = HashMap::with_capacity(history_rows.len());
    for row in history_rows {
        history_map.insert(row.card_id, deserialize_history(&row.items)?);
    }

    Ok(history_map)
}

pub(crate) fn to_card(model: card_model::Model, history: Vec<CardHistory>) -> Card {
    Card {
        id: model.id,
        title: model.title,
        description: model.description,
        deadline: model.deadline,
        deleted: model.deleted,
        project_id: model.project_id,
        section_id: model.section_id,
        history,
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph::{
    AppErr, Card, CardCommands, CardQueries, Database, DatabaseBuilder, Event, EventCommands,
    EventQueries, Project, ProjectCommands, ProjectQueries, SqlxCardQueries, SqlxDatabase,
    SqlxDatabaseBuilder, SqlxEventQueries, SqlxProjectQueries, parse_date, parse_optional_date,
};
use serde::Serialize;
use tauri::{State, async_runtime::block_on};

type DatabaseAdapter = SqlxDatabase;
type DatabaseBuilderAdapter = SqlxDatabaseBuilder;
type CardQueryAdapter = SqlxCardQueries;
type EventQueryAdapter = SqlxEventQueries;
type ProjectQueryAdapter = SqlxProjectQueries;
type TauriResult<T> = Result<T, TauriErr>;

#[derive(Clone, Debug, Serialize)]
struct TauriErr(String);

impl From<AppErr> for TauriErr {
    fn from(err: AppErr) -> Self {
        Self(err.to_string())
    }
}

fn parse_uuid(value: &str, field: &str) -> TauriResult<uuid::Uuid> {
    value
        .parse()
        .map_err(|err: uuid::Error| TauriErr(format!("Invalid UUID for {field}: {err}")))
}

fn parse_optional_uuid(value: Option<String>, field: &str) -> TauriResult<Option<uuid::Uuid>> {
    value.as_deref().map(|id| parse_uuid(id, field)).transpose()
}

struct AppState {
    db: DatabaseAdapter,
}

#[tauri::command]
async fn get_cards(state: State<'_, AppState>) -> TauriResult<Vec<Card>> {
    Ok(CardQueryAdapter::new(state.db.conn())
        .get_all_cards()
        .await?)
}

#[tauri::command]
async fn get_cards_without_project(state: State<'_, AppState>) -> TauriResult<Vec<Card>> {
    Ok(CardQueryAdapter::new(state.db.conn())
        .get_cards_without_project()
        .await?)
}

#[tauri::command]
async fn get_cards_by_project(
    project_id: String,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Card>> {
    let project_id = parse_uuid(&project_id, "project_id")?;
    Ok(CardQueryAdapter::new(state.db.conn())
        .get_cards_by_project(project_id)
        .await?)
}

#[tauri::command]
async fn add_card(
    title: String,
    project_id: Option<String>,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Card>> {
    let project_id = parse_optional_uuid(project_id, "project_id")?;
    state
        .db
        .begin(async |uow| {
            match project_id {
                Some(pid) => {
                    CardCommands::new(uow).add_with_project(&title, pid).await?;
                }
                None => {
                    CardCommands::new(uow).add(&title).await?;
                }
            }
            Ok(())
        })
        .await?;
    Ok(CardQueryAdapter::new(state.db.conn())
        .get_all_cards()
        .await?)
}

#[tauri::command]
async fn toggle_card(id: String, state: State<'_, AppState>) -> TauriResult<Vec<Card>> {
    let id = parse_uuid(&id, "card id")?;
    state
        .db
        .begin(async |uow| CardCommands::new(uow).toggle(id).await)
        .await?;
    Ok(CardQueryAdapter::new(state.db.conn())
        .get_all_cards()
        .await?)
}

#[tauri::command]
async fn delete_card(id: String, state: State<'_, AppState>) -> TauriResult<Vec<Card>> {
    let id = parse_uuid(&id, "card id")?;
    state
        .db
        .begin(async |uow| CardCommands::new(uow).delete(id).await)
        .await?;
    Ok(CardQueryAdapter::new(state.db.conn())
        .get_all_cards()
        .await?)
}

#[tauri::command]
async fn update_card(
    id: String,
    title: String,
    description: String,
    deadline: Option<String>,
    state: State<'_, AppState>,
) -> TauriResult<()> {
    let id = parse_uuid(&id, "card id")?;
    let deadline = parse_optional_date(deadline.as_deref())?;
    state
        .db
        .begin(async |uow| {
            CardCommands::new(uow)
                .edit(id, &title, &description, deadline)
                .await
        })
        .await?;
    Ok(())
}

#[tauri::command]
async fn get_events(state: State<'_, AppState>) -> TauriResult<Vec<Event>> {
    Ok(EventQueryAdapter::new(state.db.conn())
        .get_all_events()
        .await?)
}

#[tauri::command]
async fn get_events_without_project(state: State<'_, AppState>) -> TauriResult<Vec<Event>> {
    Ok(EventQueryAdapter::new(state.db.conn())
        .get_events_without_project()
        .await?)
}

#[tauri::command]
async fn get_events_by_project(
    project_id: String,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Event>> {
    let project_id = parse_uuid(&project_id, "project_id")?;
    Ok(EventQueryAdapter::new(state.db.conn())
        .get_events_by_project(project_id)
        .await?)
}

#[tauri::command]
async fn add_event(
    date: String,
    title: String,
    description: String,
    project_id: Option<String>,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Event>> {
    let date = parse_date(&date)?;
    let project_id = parse_optional_uuid(project_id, "project_id")?;

    state
        .db
        .begin(async |uow| {
            match project_id {
                Some(pid) => {
                    EventCommands::new(uow)
                        .add_with_project(date, &title, &description, pid)
                        .await?;
                }
                None => {
                    EventCommands::new(uow)
                        .add(date, &title, &description)
                        .await?;
                }
            }
            Ok(())
        })
        .await?;

    Ok(EventQueryAdapter::new(state.db.conn())
        .get_all_events()
        .await?)
}

#[tauri::command]
async fn update_event(
    id: String,
    date: String,
    title: String,
    description: String,
    state: State<'_, AppState>,
) -> TauriResult<()> {
    let id = parse_uuid(&id, "event id")?;
    let date = parse_date(&date)?;
    state
        .db
        .begin(async |uow| {
            EventCommands::new(uow)
                .edit(id, date, &title, &description)
                .await
        })
        .await?;
    Ok(())
}

#[tauri::command]
async fn get_projects(state: State<'_, AppState>) -> TauriResult<Vec<Project>> {
    Ok(ProjectQueryAdapter::new(state.db.conn())
        .get_all_projects()
        .await?)
}

#[tauri::command]
async fn add_project(title: String, state: State<'_, AppState>) -> TauriResult<Vec<Project>> {
    state
        .db
        .begin(async |uow| {
            ProjectCommands::new(uow).add(&title).await?;
            Ok(())
        })
        .await?;
    Ok(ProjectQueryAdapter::new(state.db.conn())
        .get_all_projects()
        .await?)
}

fn main() {
    let db = block_on(
        DatabaseBuilderAdapter::open("../db.sqlite")
            .migrate()
            .finish(),
    )
    .expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(AppState { db })
        .invoke_handler(tauri::generate_handler![
            get_cards,
            get_cards_without_project,
            get_cards_by_project,
            add_card,
            toggle_card,
            delete_card,
            update_card,
            get_events,
            get_events_without_project,
            get_events_by_project,
            add_event,
            update_event,
            get_projects,
            add_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph::{
    AppErr, Card, CardCommands, CardQueries, Database, DatabaseBuilder, Project, ProjectCommands,
    ProjectData, ProjectQueries, QueryFilter, SeaOrmDatabase, SeaOrmDatabaseBuilder, Section,
    SectionCommands, SectionQueries, parse_date, parse_optional_date,
};
use serde::Serialize;
use tauri::{State, async_runtime::block_on};

type DatabaseAdapter = SeaOrmDatabase;
type DatabaseBuilderAdapter = SeaOrmDatabaseBuilder;
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

// Tauri injects `State<'_, AppState>` per command invocation. Each `'_` below
// is that request-scoped borrow of the managed application state.
#[tauri::command]
async fn filter_cards(
    limit: u32,
    offset: u32,
    deadline: QueryFilter<String>,
    project_id: QueryFilter<String>,
    section_id: QueryFilter<String>,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Card>> {
    let deadline = deadline.try_map(|date| parse_date(&date).map_err(TauriErr::from))?;
    let project_id = project_id.try_map(|id| parse_uuid(&id, "project_id"))?;
    let section_id = section_id.try_map(|id| parse_uuid(&id, "section_id"))?;

    Ok(state
        .db
        .card()
        .filter(limit, offset, deadline, project_id, section_id)
        .await?)
}

#[tauri::command]
async fn add_card(
    title: String,
    project_id: Option<String>,
    section_id: Option<String>,
    state: State<'_, AppState>,
) -> TauriResult<()> {
    let project_id = parse_optional_uuid(project_id, "project_id")?;
    let section_id = parse_optional_uuid(section_id, "section_id")?;
    state
        .db
        .begin(async |uow| {
            CardCommands::new(uow)
                .add_with_assignment(&title, project_id, section_id)
                .await?;
            Ok(())
        })
        .await?;
    Ok(())
}

#[tauri::command]
async fn toggle_card(id: String, state: State<'_, AppState>) -> TauriResult<()> {
    let id = parse_uuid(&id, "card id")?;
    state
        .db
        .begin(async |uow| CardCommands::new(uow).toggle(id).await)
        .await?;
    Ok(())
}

#[tauri::command]
async fn delete_card(id: String, state: State<'_, AppState>) -> TauriResult<()> {
    let id = parse_uuid(&id, "card id")?;
    state
        .db
        .begin(async |uow| CardCommands::new(uow).delete(id).await)
        .await?;
    Ok(())
}

#[tauri::command]
async fn update_card(
    id: String,
    title: String,
    description: String,
    deadline: Option<String>,
    project_id: Option<String>,
    section_id: Option<String>,
    state: State<'_, AppState>,
) -> TauriResult<()> {
    let id = parse_uuid(&id, "card id")?;
    let deadline = parse_optional_date(deadline.as_deref())?;
    let project_id = parse_optional_uuid(project_id, "project_id")?;
    let section_id = parse_optional_uuid(section_id, "section_id")?;
    state
        .db
        .begin(async |uow| {
            CardCommands::new(uow)
                .edit(id, &title, &description, deadline, project_id, section_id)
                .await
        })
        .await?;
    Ok(())
}

#[tauri::command]
async fn filter_projects(
    limit: u32,
    offset: u32,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Project>> {
    Ok(state.db.project().filter(limit, offset).await?)
}

#[tauri::command]
async fn get_project(
    project_id: String,
    state: State<'_, AppState>,
) -> TauriResult<Option<ProjectData>> {
    let project_id = parse_uuid(&project_id, "project_id")?;
    Ok(state.db.project().get_project(project_id).await?)
}

#[tauri::command]
async fn add_project(title: String, state: State<'_, AppState>) -> TauriResult<()> {
    state
        .db
        .begin(async |uow| {
            ProjectCommands::new(uow).add(&title).await?;
            Ok(())
        })
        .await?;
    Ok(())
}

#[tauri::command]
async fn filter_sections(
    limit: u32,
    offset: u32,
    project_id: QueryFilter<String>,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Section>> {
    let project_id = project_id.try_map(|id| parse_uuid(&id, "project_id"))?;

    Ok(state.db.section().filter(limit, offset, project_id).await?)
}

#[tauri::command]
async fn add_section(
    title: String,
    project_id: String,
    state: State<'_, AppState>,
) -> TauriResult<()> {
    let project_id = parse_uuid(&project_id, "project_id")?;
    state
        .db
        .begin(async |uow| {
            SectionCommands::new(uow).add(&title, project_id).await?;
            Ok(())
        })
        .await?;
    Ok(())
}

#[tauri::command]
async fn update_section(id: String, title: String, state: State<'_, AppState>) -> TauriResult<()> {
    let id = parse_uuid(&id, "section id")?;
    state
        .db
        .begin(async |uow| SectionCommands::new(uow).edit(id, &title).await)
        .await?;
    Ok(())
}

#[tauri::command]
async fn delete_section(id: String, state: State<'_, AppState>) -> TauriResult<()> {
    let id = parse_uuid(&id, "section id")?;
    state
        .db
        .begin(async |uow| SectionCommands::new(uow).delete(id).await)
        .await?;
    Ok(())
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
            filter_cards,
            add_card,
            toggle_card,
            delete_card,
            update_card,
            filter_projects,
            get_project,
            add_project,
            filter_sections,
            add_section,
            update_section,
            delete_section,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

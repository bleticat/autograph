#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{
    AppErr, Database, Project, ProjectCommands, ProjectQueries, SqlxDatabase,
    SqlxProjectQueries, SqlxTaskQueries, TaskCommands, TaskQueries, Todo,
};
use serde::Serialize;
use tauri::{State, async_runtime::block_on};

type DatabaseAdapter = SqlxDatabase;
type TaskQueryAdapter = SqlxTaskQueries;
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
async fn get_todos(state: State<'_, AppState>) -> TauriResult<Vec<Todo>> {
    Ok(TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await?)
}

#[tauri::command]
async fn get_todos_without_project(state: State<'_, AppState>) -> TauriResult<Vec<Todo>> {
    Ok(TaskQueryAdapter::new(state.db.conn())
        .get_todos_without_project()
        .await?)
}

#[tauri::command]
async fn get_todos_by_project(
    project_id: String,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Todo>> {
    let project_id = parse_uuid(&project_id, "project_id")?;
    Ok(TaskQueryAdapter::new(state.db.conn())
        .get_todos_by_project(project_id)
        .await?)
}

#[tauri::command]
async fn add_todo(
    title: String,
    project_id: Option<String>,
    state: State<'_, AppState>,
) -> TauriResult<Vec<Todo>> {
    let project_id = parse_optional_uuid(project_id, "project_id")?;
    state
        .db
        .begin(async |uow| {
            match project_id {
                Some(pid) => {
                    TaskCommands::new(uow).add_with_project(&title, pid).await?;
                }
                None => {
                    TaskCommands::new(uow).add(&title).await?;
                }
            }
            Ok(())
        })
        .await?;
    Ok(TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await?)
}

#[tauri::command]
async fn toggle_todo(id: String, state: State<'_, AppState>) -> TauriResult<Vec<Todo>> {
    let id = parse_uuid(&id, "todo id")?;
    state
        .db
        .begin(async |uow| TaskCommands::new(uow).toggle(id).await)
        .await?;
    Ok(TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await?)
}

#[tauri::command]
async fn delete_todo(id: String, state: State<'_, AppState>) -> TauriResult<Vec<Todo>> {
    let id = parse_uuid(&id, "todo id")?;
    state
        .db
        .begin(async |uow| TaskCommands::new(uow).delete(id).await)
        .await?;
    Ok(TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await?)
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
    let db =
        block_on(DatabaseAdapter::open("../db.sqlite")).expect("Failed to initialize database");
    block_on(db.migrate()).expect("Failed to migrate database");

    tauri::Builder::default()
        .manage(AppState { db })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            get_todos_without_project,
            get_todos_by_project,
            add_todo,
            toggle_todo,
            delete_todo,
            get_projects,
            add_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

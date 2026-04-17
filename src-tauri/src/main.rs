#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{
    Database, Project, ProjectCommands, ProjectQueries, SqliteProjectQueries, SqliteTaskQueries,
    SqlxDatabase, TaskCommands, TaskQueries, Todo,
};
use tauri::{State, async_runtime::block_on};

type DatabaseAdapter = SqlxDatabase;
type TaskQueryAdapter = SqliteTaskQueries;
type ProjectQueryAdapter = SqliteProjectQueries;

struct AppState {
    db: DatabaseAdapter,
}

#[tauri::command]
async fn get_todos(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_todos_without_project(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    TaskQueryAdapter::new(state.db.conn())
        .get_todos_without_project()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_todos_by_project(
    project_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Todo>, String> {
    let project_id = project_id
        .parse()
        .map_err(|e| format!("Invalid UUID for project_id: {e}"))?;
    TaskQueryAdapter::new(state.db.conn())
        .get_todos_by_project(project_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_todo(
    title: String,
    project_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Todo>, String> {
    let project_id = project_id
        .map(|id| {
            id.parse()
                .map_err(|e| format!("Invalid UUID for project_id: {e}"))
        })
        .transpose()?;
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
        .await
        .map_err(|e| e.to_string())?;
    TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_todo(id: String, state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let id = id
        .parse()
        .map_err(|e| format!("Invalid UUID for todo id: {e}"))?;
    state
        .db
        .begin(async |uow| TaskCommands::new(uow).toggle(id).await)
        .await
        .map_err(|e| e.to_string())?;
    TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_todo(id: String, state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let id = id
        .parse()
        .map_err(|e| format!("Invalid UUID for todo id: {e}"))?;
    state
        .db
        .begin(async |uow| TaskCommands::new(uow).delete(id).await)
        .await
        .map_err(|e| e.to_string())?;
    TaskQueryAdapter::new(state.db.conn())
        .get_all_todos()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    ProjectQueryAdapter::new(state.db.conn())
        .get_all_projects()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project(title: String, state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state
        .db
        .begin(async |uow| {
            ProjectCommands::new(uow).add(&title).await?;
            Ok(())
        })
        .await
        .map_err(|e| e.to_string())?;
    ProjectQueryAdapter::new(state.db.conn())
        .get_all_projects()
        .await
        .map_err(|e| e.to_string())
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

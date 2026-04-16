#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{
    Database, Project, ProjectCommands, ProjectQueries, Queries, Repository, SqliteProjectQueries,
    SqliteProjectRepository, SqliteTaskQueries, SqliteTodoRepository, SqlxDatabase, TaskCommands,
    TaskQueries, Todo,
};
use std::sync::Mutex;
use tauri::{async_runtime::block_on, State};

type Db = SqlxDatabase;
type QueryAdapter = SqliteTaskQueries;
type ProjectQueryAdapter = SqliteProjectQueries;

struct AppState {
    db: Mutex<Db>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    block_on(QueryAdapter::from(db.conn()).get_all_todos()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_todos_without_project(state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    block_on(QueryAdapter::from(db.conn()).get_todos_without_project()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_todos_by_project(project_id: String, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let project_id = project_id
        .parse()
        .map_err(|e| format!("Invalid UUID for project_id: {e}"))?;
    let db = state.db.lock().unwrap();
    block_on(QueryAdapter::from(db.conn()).get_todos_by_project(project_id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_todo(
    title: String,
    project_id: Option<String>,
    state: State<AppState>,
) -> Result<Vec<Todo>, String> {
    let project_id = project_id
        .map(|id| {
            id.parse()
                .map_err(|e| format!("Invalid UUID for project_id: {e}"))
        })
        .transpose()?;
    let db = state.db.lock().unwrap();
    block_on(db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        match project_id {
            Some(pid) => TaskCommands::new(&repo).add_with_project(&title, pid).await,
            None => TaskCommands::new(&repo).add(&title).await,
        }
    }))
    .map_err(|e| e.to_string())?;
    block_on(QueryAdapter::from(db.conn()).get_all_todos()).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_todo(id: String, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let id = id
        .parse()
        .map_err(|e| format!("Invalid UUID for todo id: {e}"))?;
    let db = state.db.lock().unwrap();
    block_on(db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo).toggle(id).await
    }))
    .map_err(|e| e.to_string())?;
    block_on(QueryAdapter::from(db.conn()).get_all_todos()).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_todo(id: String, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let id = id
        .parse()
        .map_err(|e| format!("Invalid UUID for todo id: {e}"))?;
    let db = state.db.lock().unwrap();
    block_on(db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo).delete(id).await
    }))
    .map_err(|e| e.to_string())?;
    block_on(QueryAdapter::from(db.conn()).get_all_todos()).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_projects(state: State<AppState>) -> Result<Vec<Project>, String> {
    let db = state.db.lock().unwrap();
    block_on(ProjectQueryAdapter::from(db.conn()).get_all_projects()).map_err(|e| e.to_string())
}

#[tauri::command]
fn add_project(title: String, state: State<AppState>) -> Result<Vec<Project>, String> {
    let db = state.db.lock().unwrap();
    block_on(db.transaction(async |tx| {
        let repo = SqliteProjectRepository::bind(tx);
        ProjectCommands::new(&repo).add(&title).await
    }))
    .map_err(|e| e.to_string())?;
    block_on(ProjectQueryAdapter::from(db.conn()).get_all_projects()).map_err(|e| e.to_string())
}

fn main() {
    let db = block_on(Db::open("../db.sqlite")).expect("Failed to initialize database");
    block_on(db.migrate()).expect("Failed to migrate database");

    tauri::Builder::default()
        .manage(AppState { db: Mutex::new(db) })
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

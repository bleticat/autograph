#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{
    Database, SqliteDatabase, SqliteTaskQueries, SqliteTodoRepository, TaskCommands, TaskQueries,
    Todo,
};
use std::sync::Mutex;
use tauri::State;

type Db = SqliteDatabase;
type QueryAdapter<'a> = SqliteTaskQueries<'a>;
type RepoAdapter<'a> = SqliteTodoRepository<'a>;

struct AppState {
    db: Mutex<Db>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    QueryAdapter::from(db.conn())
        .get_all_todos()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn add_todo(title: String, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    db.transaction(|tx| {
        let repo = RepoAdapter::from(tx);
        TaskCommands::new(&repo).add(&title)
    })
    .map_err(|e| e.to_string())?;
    QueryAdapter::from(db.conn())
        .get_all_todos()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_todo(id: i64, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    db.transaction(|tx| {
        let repo = RepoAdapter::from(tx);
        TaskCommands::new(&repo).toggle(id)
    })
    .map_err(|e| e.to_string())?;
    QueryAdapter::from(db.conn())
        .get_all_todos()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_todo(id: i64, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    db.transaction(|tx| {
        let repo = RepoAdapter::from(tx);
        TaskCommands::new(&repo).delete(id)
    })
    .map_err(|e| e.to_string())?;
    QueryAdapter::from(db.conn())
        .get_all_todos()
        .map_err(|e| e.to_string())
}

fn main() {
    let db = Db::open("../db.sqlite").expect("Failed to initialize database");
    db.migrate().expect("Failed to migrate database");

    tauri::Builder::default()
        .manage(AppState { db: Mutex::new(db) })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

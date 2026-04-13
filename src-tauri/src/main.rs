#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{
    Database, SqliteDatabase, SqliteTaskQueries, SqliteTodoRepository, TaskCommands, TaskQueries,
    Todo, TodoRepository,
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
    QueryAdapter::new(db.conn()).get_all_todos()
}

#[tauri::command]
fn add_todo(title: String, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    db.transaction(|tx| {
        let repo = RepoAdapter::new(tx);
        TaskCommands::new(&repo).add(&title)
    })?;
    QueryAdapter::new(db.conn()).get_all_todos()
}

#[tauri::command]
fn toggle_todo(id: i64, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    db.transaction(|tx| {
        let repo = RepoAdapter::new(tx);
        TaskCommands::new(&repo).toggle(id)
    })?;
    QueryAdapter::new(db.conn()).get_all_todos()
}

#[tauri::command]
fn delete_todo(id: i64, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let db = state.db.lock().unwrap();
    db.transaction(|tx| {
        let repo = RepoAdapter::new(tx);
        TaskCommands::new(&repo).delete(id)
    })?;
    QueryAdapter::new(db.conn()).get_all_todos()
}

fn main() {
    let db = Db::open("../db.sqlite").expect("Failed to initialize database");

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

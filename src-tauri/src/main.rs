#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{commands, queries, SqliteTodoRepository, Todo};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    repo: Mutex<SqliteTodoRepository>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Result<Vec<Todo>, String> {
    let repo = state.repo.lock().unwrap();
    queries::get_all_todos(&*repo)
}

#[tauri::command]
fn add_todo(title: String, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let repo = state.repo.lock().unwrap();
    commands::add_todo(&*repo, &title)?;
    queries::get_all_todos(&*repo)
}

#[tauri::command]
fn toggle_todo(id: i64, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let repo = state.repo.lock().unwrap();
    commands::toggle_todo(&*repo, id)?;
    queries::get_all_todos(&*repo)
}

#[tauri::command]
fn delete_todo(id: i64, state: State<AppState>) -> Result<Vec<Todo>, String> {
    let repo = state.repo.lock().unwrap();
    commands::delete_todo(&*repo, id)?;
    queries::get_all_todos(&*repo)
}

fn main() {
    let repo = SqliteTodoRepository::open("../db.sqlite").expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(AppState {
            repo: Mutex::new(repo),
        })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            toggle_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

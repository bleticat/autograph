#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autograph_core::{Database, Todo};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    db: Mutex<Database>,
}

#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<Todo> {
    let db = state.db.lock().unwrap();
    db.get_all().unwrap_or_default()
}

#[tauri::command]
fn add_todo(title: String, state: State<AppState>) -> Vec<Todo> {
    let db = state.db.lock().unwrap();
    let _ = db.add(&title);
    db.get_all().unwrap_or_default()
}

#[tauri::command]
fn toggle_todo(id: i64, state: State<AppState>) -> Vec<Todo> {
    let db = state.db.lock().unwrap();
    let _ = db.toggle(id);
    db.get_all().unwrap_or_default()
}

#[tauri::command]
fn delete_todo(id: i64, state: State<AppState>) -> Vec<Todo> {
    let db = state.db.lock().unwrap();
    let _ = db.delete(id);
    db.get_all().unwrap_or_default()
}

fn main() {
    let db = Database::open("../db.sqlite").expect("Failed to initialize database");

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

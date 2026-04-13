use autograph_core::{commands, queries, SqliteTodoRepository};

fn fresh_repo() -> SqliteTodoRepository {
    SqliteTodoRepository::in_memory().expect("failed to create in-memory repo")
}

#[test]
fn empty_database_returns_no_todos() {
    let repo = fresh_repo();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert!(todos.is_empty());
}

#[test]
fn add_single_todo() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "buy milk").unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "buy milk");
    assert!(!todos[0].completed);
}

#[test]
fn add_multiple_todos_preserves_order() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "first").unwrap();
    commands::add_todo(&repo, "second").unwrap();
    commands::add_todo(&repo, "third").unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].title, "first");
    assert_eq!(todos[1].title, "second");
    assert_eq!(todos[2].title, "third");
}

#[test]
fn toggle_marks_completed() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "task").unwrap();
    let id = queries::get_all_todos(&repo).unwrap()[0].id;

    commands::toggle_todo(&repo, id).unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert!(todos[0].completed);
}

#[test]
fn toggle_twice_restores_incomplete() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "task").unwrap();
    let id = queries::get_all_todos(&repo).unwrap()[0].id;

    commands::toggle_todo(&repo, id).unwrap();
    commands::toggle_todo(&repo, id).unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert!(!todos[0].completed);
}

#[test]
fn delete_removes_todo() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "to delete").unwrap();
    let id = queries::get_all_todos(&repo).unwrap()[0].id;

    commands::delete_todo(&repo, id).unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert!(todos.is_empty());
}

#[test]
fn delete_only_target_todo() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "keep").unwrap();
    commands::add_todo(&repo, "remove").unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    let remove_id = todos[1].id;

    commands::delete_todo(&repo, remove_id).unwrap();
    let todos = queries::get_all_todos(&repo).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "keep");
}

#[test]
fn toggle_nonexistent_id_is_noop() {
    let repo = fresh_repo();
    commands::toggle_todo(&repo, 9999).unwrap();
    assert!(queries::get_all_todos(&repo).unwrap().is_empty());
}

#[test]
fn delete_nonexistent_id_is_noop() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "still here").unwrap();
    commands::delete_todo(&repo, 9999).unwrap();
    assert_eq!(queries::get_all_todos(&repo).unwrap().len(), 1);
}

#[test]
fn ids_are_unique_after_delete() {
    let repo = fresh_repo();
    commands::add_todo(&repo, "first").unwrap();
    let first_id = queries::get_all_todos(&repo).unwrap()[0].id;
    commands::delete_todo(&repo, first_id).unwrap();

    commands::add_todo(&repo, "second").unwrap();
    let second_id = queries::get_all_todos(&repo).unwrap()[0].id;
    assert_ne!(first_id, second_id);
}

#[test]
fn full_workflow() {
    let repo = fresh_repo();

    // Add a few todos
    commands::add_todo(&repo, "buy groceries").unwrap();
    commands::add_todo(&repo, "write tests").unwrap();
    commands::add_todo(&repo, "deploy app").unwrap();

    // Complete one
    let todos = queries::get_all_todos(&repo).unwrap();
    commands::toggle_todo(&repo, todos[1].id).unwrap();

    // Delete one
    commands::delete_todo(&repo, todos[2].id).unwrap();

    // Verify final state
    let todos = queries::get_all_todos(&repo).unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "buy groceries");
    assert!(!todos[0].completed);
    assert_eq!(todos[1].title, "write tests");
    assert!(todos[1].completed);
}

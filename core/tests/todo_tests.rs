use autograph_core::Database;

fn fresh_db() -> Database {
    Database::in_memory().expect("failed to create in-memory db")
}

#[test]
fn empty_database_returns_no_todos() {
    let db = fresh_db();
    let todos = db.get_all().unwrap();
    assert!(todos.is_empty());
}

#[test]
fn add_single_todo() {
    let db = fresh_db();
    db.add("buy milk").unwrap();
    let todos = db.get_all().unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "buy milk");
    assert!(!todos[0].completed);
}

#[test]
fn add_multiple_todos_preserves_order() {
    let db = fresh_db();
    db.add("first").unwrap();
    db.add("second").unwrap();
    db.add("third").unwrap();
    let todos = db.get_all().unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].title, "first");
    assert_eq!(todos[1].title, "second");
    assert_eq!(todos[2].title, "third");
}

#[test]
fn toggle_marks_completed() {
    let db = fresh_db();
    db.add("task").unwrap();
    let id = db.get_all().unwrap()[0].id;

    db.toggle(id).unwrap();
    let todos = db.get_all().unwrap();
    assert!(todos[0].completed);
}

#[test]
fn toggle_twice_restores_incomplete() {
    let db = fresh_db();
    db.add("task").unwrap();
    let id = db.get_all().unwrap()[0].id;

    db.toggle(id).unwrap();
    db.toggle(id).unwrap();
    let todos = db.get_all().unwrap();
    assert!(!todos[0].completed);
}

#[test]
fn delete_removes_todo() {
    let db = fresh_db();
    db.add("to delete").unwrap();
    let id = db.get_all().unwrap()[0].id;

    db.delete(id).unwrap();
    let todos = db.get_all().unwrap();
    assert!(todos.is_empty());
}

#[test]
fn delete_only_target_todo() {
    let db = fresh_db();
    db.add("keep").unwrap();
    db.add("remove").unwrap();
    let todos = db.get_all().unwrap();
    let remove_id = todos[1].id;

    db.delete(remove_id).unwrap();
    let todos = db.get_all().unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "keep");
}

#[test]
fn toggle_nonexistent_id_is_noop() {
    let db = fresh_db();
    db.toggle(9999).unwrap();
    assert!(db.get_all().unwrap().is_empty());
}

#[test]
fn delete_nonexistent_id_is_noop() {
    let db = fresh_db();
    db.add("still here").unwrap();
    db.delete(9999).unwrap();
    assert_eq!(db.get_all().unwrap().len(), 1);
}

#[test]
fn ids_are_unique_after_delete() {
    let db = fresh_db();
    db.add("first").unwrap();
    let first_id = db.get_all().unwrap()[0].id;
    db.delete(first_id).unwrap();

    db.add("second").unwrap();
    let second_id = db.get_all().unwrap()[0].id;
    assert_ne!(first_id, second_id);
}

#[test]
fn full_workflow() {
    let db = fresh_db();

    // Add a few todos
    db.add("buy groceries").unwrap();
    db.add("write tests").unwrap();
    db.add("deploy app").unwrap();

    // Complete one
    let todos = db.get_all().unwrap();
    db.toggle(todos[1].id).unwrap();

    // Delete one
    db.delete(todos[2].id).unwrap();

    // Verify final state
    let todos = db.get_all().unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "buy groceries");
    assert!(!todos[0].completed);
    assert_eq!(todos[1].title, "write tests");
    assert!(todos[1].completed);
}

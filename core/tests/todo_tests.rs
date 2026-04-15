use autograph_core::{
    Database, SqliteTaskQueries, SqliteTodoRepository, SqlxDatabase, TaskCommands, TaskQueries,
};
use futures::executor::block_on;
use uuid::Uuid;

fn fresh_db() -> SqlxDatabase {
    let db = SqlxDatabase::open(":memory:").expect("failed to create in-memory db");
    db.migrate().expect("failed to run migrations");
    db
}

#[test]
fn empty_database_returns_no_todos() {
    let db = fresh_db();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(todos.is_empty());
}

#[test]
fn add_single_todo() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("buy milk"))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "buy milk");
    assert!(!todos[0].completed);
}

#[test]
fn add_multiple_todos_preserves_order() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("first"))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("second"))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("third"))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].title, "first");
    assert_eq!(todos[1].title, "second");
    assert_eq!(todos[2].title, "third");
}

#[test]
fn toggle_marks_completed() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("task"))
    })
    .unwrap();
    let id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;

    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).toggle(id))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(todos[0].completed);
}

#[test]
fn toggle_twice_restores_incomplete() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("task"))
    })
    .unwrap();
    let id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;

    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).toggle(id))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).toggle(id))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(!todos[0].completed);
}

#[test]
fn delete_removes_todo() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("to delete"))
    })
    .unwrap();
    let id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;

    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).delete(id))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(todos.is_empty());
}

#[test]
fn delete_only_target_todo() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("keep"))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("remove"))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    let remove_id = todos[1].id;

    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).delete(remove_id))
    })
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "keep");
}

#[test]
fn toggle_nonexistent_id_is_noop() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).toggle(Uuid::new_v4()))
    })
    .unwrap();
    assert!(block_on(SqliteTaskQueries::from(db.conn()).get_all_todos())
        .unwrap()
        .is_empty());
}

#[test]
fn delete_nonexistent_id_is_noop() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("still here"))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).delete(Uuid::new_v4()))
    })
    .unwrap();
    assert_eq!(
        block_on(SqliteTaskQueries::from(db.conn()).get_all_todos())
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn ids_are_unique_after_delete() {
    let db = fresh_db();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("first"))
    })
    .unwrap();
    let first_id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).delete(first_id))
    })
    .unwrap();

    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("second"))
    })
    .unwrap();
    let second_id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;
    assert_ne!(first_id, second_id);
}

#[test]
fn full_workflow() {
    let db = fresh_db();

    // Add a few todos
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("buy groceries"))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("write tests"))
    })
    .unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).add("deploy app"))
    })
    .unwrap();

    // Complete one
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).toggle(todos[1].id))
    })
    .unwrap();

    // Delete one
    db.transaction(|tx| {
        let repo = SqliteTodoRepository::from(tx);
        block_on(TaskCommands::new(&repo).delete(todos[2].id))
    })
    .unwrap();

    // Verify final state
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "buy groceries");
    assert!(!todos[0].completed);
    assert_eq!(todos[1].title, "write tests");
    assert!(todos[1].completed);
}

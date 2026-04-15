use autograph_core::{
    Database, SqliteTaskQueries, SqliteTodoRepository, SqlxDatabase, TaskCommands, TaskQueries,
};
use futures::executor::block_on;
use uuid::Uuid;

fn fresh_db() -> SqlxDatabase {
    let db = block_on(SqlxDatabase::open(":memory:")).expect("failed to create in-memory db");
    block_on(db.migrate()).expect("failed to run migrations");
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
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("buy milk").await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "buy milk");
    assert!(!todos[0].completed);
}

#[test]
fn add_multiple_todos_preserves_order() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("first").await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("second").await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("third").await
    }))
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
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("task").await
    }))
    .unwrap();
    let id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).toggle(id).await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(todos[0].completed);
}

#[test]
fn toggle_twice_restores_incomplete() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("task").await
    }))
    .unwrap();
    let id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).toggle(id).await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).toggle(id).await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(!todos[0].completed);
}

#[test]
fn delete_removes_todo() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("to delete").await
    }))
    .unwrap();
    let id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).delete(id).await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert!(todos.is_empty());
}

#[test]
fn delete_only_target_todo() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("keep").await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("remove").await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    let remove_id = todos[1].id;

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).delete(remove_id).await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "keep");
}

#[test]
fn toggle_nonexistent_id_is_noop() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).toggle(Uuid::new_v4()).await
    }))
    .unwrap();
    assert!(block_on(SqliteTaskQueries::from(db.conn()).get_all_todos())
        .unwrap()
        .is_empty());
}

#[test]
fn delete_nonexistent_id_is_noop() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("still here").await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).delete(Uuid::new_v4()).await
    }))
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
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("first").await
    }))
    .unwrap();
    let first_id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).delete(first_id).await
    }))
    .unwrap();

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("second").await
    }))
    .unwrap();
    let second_id = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap()[0].id;
    assert_ne!(first_id, second_id);
}

#[test]
fn full_workflow() {
    let db = fresh_db();

    // Add a few todos
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("buy groceries").await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("write tests").await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("deploy app").await
    }))
    .unwrap();

    // Complete one
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    let toggle_id = todos[1].id;
    let delete_id = todos[2].id;
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).toggle(toggle_id).await
    }))
    .unwrap();

    // Delete one
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).delete(delete_id).await
    }))
    .unwrap();

    // Verify final state
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_all_todos()).unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "buy groceries");
    assert!(!todos[0].completed);
    assert_eq!(todos[1].title, "write tests");
    assert!(todos[1].completed);
}

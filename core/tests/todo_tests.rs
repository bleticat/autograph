use autograph_core::{
    Database, SqlxDatabase, SqlxTaskQueries, SqlxUnitOfWork, TaskCommands, TaskQueries, Todo,
};
use autograph_core::shared::ports::repository::Repository;
use time::{Date, Month, Time};
use uuid::Uuid;

async fn fresh_db() -> SqlxDatabase {
    let db = (SqlxDatabase::open(":memory:"))
        .await
        .expect("failed to create in-memory db");
    (db.migrate()).await.expect("failed to run migrations");
    db
}

#[tokio::test]
async fn empty_database_returns_no_todos() {
    let db = fresh_db().await;
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert!(todos.is_empty());
}

#[tokio::test]
async fn add_single_todo() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("buy milk").await)
        .await
        .unwrap();
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "buy milk");
    assert_eq!(todos[0].description, "");
    assert_eq!(todos[0].deadline, None);
    assert!(!todos[0].completed);
}

#[tokio::test]
async fn add_multiple_todos_preserves_order() {
    let db = fresh_db().await;
    for title in ["first", "second", "third"] {
        db.begin(async |uow| TaskCommands::new(uow).add(title).await)
            .await
            .unwrap();
    }
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0].title, "first");
    assert_eq!(todos[1].title, "second");
    assert_eq!(todos[2].title, "third");
}

#[tokio::test]
async fn toggle_marks_completed() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("task").await)
        .await
        .unwrap();
    let id = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| TaskCommands::new(uow).toggle(id).await)
        .await
        .unwrap();
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert!(todos[0].completed);
}

#[tokio::test]
async fn toggle_twice_restores_incomplete() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("task").await)
        .await
        .unwrap();
    let id = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| TaskCommands::new(uow).toggle(id).await)
        .await
        .unwrap();
    db.begin(async |uow| TaskCommands::new(uow).toggle(id).await)
        .await
        .unwrap();
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert!(!todos[0].completed);
}

#[tokio::test]
async fn delete_removes_todo() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("to delete").await)
        .await
        .unwrap();
    let id = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| TaskCommands::new(uow).delete(id).await)
        .await
        .unwrap();
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert!(todos.is_empty());
}

#[tokio::test]
async fn delete_only_target_todo() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("keep").await)
        .await
        .unwrap();
    db.begin(async |uow| TaskCommands::new(uow).add("remove").await)
        .await
        .unwrap();
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    let remove_id = todos[1].id;

    db.begin(async |uow| TaskCommands::new(uow).delete(remove_id).await)
        .await
        .unwrap();
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "keep");
}

#[tokio::test]
async fn toggle_nonexistent_id_is_noop() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).toggle(Uuid::new_v4()).await)
        .await
        .unwrap();
    assert!(
        (SqlxTaskQueries::new(db.conn()).get_all_todos())
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn delete_nonexistent_id_is_noop() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("still here").await)
        .await
        .unwrap();
    db.begin(async |uow| TaskCommands::new(uow).delete(Uuid::new_v4()).await)
        .await
        .unwrap();
    assert_eq!(
        (SqlxTaskQueries::new(db.conn()).get_all_todos())
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn edit_updates_task_fields() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("draft").await)
        .await
        .unwrap();
    let id = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| {
        TaskCommands::new(uow)
            .edit(
                id,
                "final title",
                "expanded task details",
                Some(
                    Date::from_calendar_date(2026, Month::May, 10)
                        .unwrap()
                        .with_time(Time::MIDNIGHT)
                        .assume_utc(),
                ),
            )
            .await
    })
    .await
    .unwrap();

    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert_eq!(todos[0].title, "final title");
    assert_eq!(todos[0].description, "expanded task details");
    assert_eq!(
        todos[0].deadline.map(|deadline| deadline.date()),
        Some(Date::from_calendar_date(2026, Month::May, 10).unwrap())
    );
}

#[tokio::test]
async fn ids_are_unique_after_delete() {
    let db = fresh_db().await;
    db.begin(async |uow| TaskCommands::new(uow).add("first").await)
        .await
        .unwrap();
    let first_id = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap()[0]
        .id;
    db.begin(async |uow| TaskCommands::new(uow).delete(first_id).await)
        .await
        .unwrap();

    db.begin(async |uow| TaskCommands::new(uow).add("second").await)
        .await
        .unwrap();
    let second_id = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap()[0]
        .id;
    assert_ne!(first_id, second_id);
}

#[tokio::test]
async fn full_workflow() {
    let db = fresh_db().await;

    // Add a few todos
    for title in ["buy groceries", "write tests", "deploy app"] {
        db.begin(async |uow| TaskCommands::new(uow).add(title).await)
            .await
            .unwrap();
    }

    // Complete one
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    let middle_todo_id = todos[1].id;
    let last_todo_id = todos[2].id;
    db.begin(async |uow| TaskCommands::new(uow).toggle(middle_todo_id).await)
        .await
        .unwrap();

    // Delete one
    db.begin(async |uow| TaskCommands::new(uow).delete(last_todo_id).await)
        .await
        .unwrap();

    // Verify final state
    let todos = (SqlxTaskQueries::new(db.conn()).get_all_todos())
        .await
        .unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "buy groceries");
    assert_eq!(todos[0].description, "");
    assert_eq!(todos[0].deadline, None);
    assert!(!todos[0].completed);
    assert_eq!(todos[1].title, "write tests");
    assert_eq!(todos[1].description, "");
    assert_eq!(todos[1].deadline, None);
    assert!(todos[1].completed);
}

#[tokio::test]
async fn repository_trait_methods_use_unit_of_work() {
    let db = fresh_db().await;

    db.begin(async |uow| {
        let todo = <SqlxUnitOfWork as Repository<Todo>>::save(
            uow,
            Todo {
                id: Uuid::nil(),
                title: "trait-backed".to_owned(),
                description: String::new(),
                deadline: None,
                completed: false,
                project_id: None,
            },
        )
        .await?;

        let fetched = <SqlxUnitOfWork as Repository<Todo>>::get(uow, todo.id).await?;
        assert_eq!(fetched.as_ref().map(|item| item.id), Some(todo.id));

        <SqlxUnitOfWork as Repository<Todo>>::delete(uow, todo.id).await?;
        Ok(())
    })
    .await
    .unwrap();

    assert!(
        (SqlxTaskQueries::new(db.conn()).get_all_todos())
            .await
            .unwrap()
            .is_empty()
    );
}

use autograph_core::{
    Database, ProjectCommands, ProjectQueries, SqliteProjectQueries, SqliteProjectRepository,
    SqliteTaskQueries, SqliteTodoRepository, SqlxDatabase, TaskCommands, TaskQueries,
};
use futures::executor::block_on;

fn fresh_db() -> SqlxDatabase {
    let db = block_on(SqlxDatabase::open(":memory:")).expect("failed to create in-memory db");
    block_on(db.migrate()).expect("failed to run migrations");
    db
}

#[test]
fn empty_database_returns_no_projects() {
    let db = fresh_db();
    let projects = block_on(SqliteProjectQueries::from(db.conn()).get_all_projects()).unwrap();
    assert!(projects.is_empty());
}

#[test]
fn add_single_project() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteProjectRepository::from(tx);
        ProjectCommands::new(&repo).add("My Project").await
    }))
    .unwrap();
    let projects = block_on(SqliteProjectQueries::from(db.conn()).get_all_projects()).unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].title, "My Project");
}

#[test]
fn add_multiple_projects_preserves_order() {
    let db = fresh_db();
    for title in &["Alpha", "Beta", "Gamma"] {
        block_on(db.transaction(|tx| async move {
            let repo = SqliteProjectRepository::from(tx);
            ProjectCommands::new(&repo).add(title).await
        }))
        .unwrap();
    }
    let projects = block_on(SqliteProjectQueries::from(db.conn()).get_all_projects()).unwrap();
    assert_eq!(projects.len(), 3);
    assert_eq!(projects[0].title, "Alpha");
    assert_eq!(projects[1].title, "Beta");
    assert_eq!(projects[2].title, "Gamma");
}

#[test]
fn todos_without_project_by_default() {
    let db = fresh_db();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("inbox task").await
    }))
    .unwrap();
    let todos = block_on(SqliteTaskQueries::from(db.conn()).get_todos_without_project()).unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "inbox task");
    assert!(todos[0].project_id.is_none());
}

#[test]
fn add_todo_with_project() {
    let db = fresh_db();
    let project_id = block_on(db.transaction(|tx| async move {
        let repo = SqliteProjectRepository::from(tx);
        ProjectCommands::new(&repo).add("Work").await
    }))
    .unwrap();

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo)
            .add_with_project("write report", project_id)
            .await
    }))
    .unwrap();

    let todos_by_project =
        block_on(SqliteTaskQueries::from(db.conn()).get_todos_by_project(project_id)).unwrap();
    assert_eq!(todos_by_project.len(), 1);
    assert_eq!(todos_by_project[0].title, "write report");
    assert_eq!(todos_by_project[0].project_id, Some(project_id));

    let todos_without =
        block_on(SqliteTaskQueries::from(db.conn()).get_todos_without_project()).unwrap();
    assert!(todos_without.is_empty());
}

#[test]
fn tasks_are_filtered_by_project() {
    let db = fresh_db();
    let p1 = block_on(db.transaction(|tx| async move {
        let repo = SqliteProjectRepository::from(tx);
        ProjectCommands::new(&repo).add("Project A").await
    }))
    .unwrap();
    let p2 = block_on(db.transaction(|tx| async move {
        let repo = SqliteProjectRepository::from(tx);
        ProjectCommands::new(&repo).add("Project B").await
    }))
    .unwrap();

    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo)
            .add_with_project("task for A", p1)
            .await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo)
            .add_with_project("task for B", p2)
            .await
    }))
    .unwrap();
    block_on(db.transaction(|tx| async move {
        let repo = SqliteTodoRepository::from(tx);
        TaskCommands::new(&repo).add("no project task").await
    }))
    .unwrap();

    let p1_todos = block_on(SqliteTaskQueries::from(db.conn()).get_todos_by_project(p1)).unwrap();
    assert_eq!(p1_todos.len(), 1);
    assert_eq!(p1_todos[0].title, "task for A");

    let p2_todos = block_on(SqliteTaskQueries::from(db.conn()).get_todos_by_project(p2)).unwrap();
    assert_eq!(p2_todos.len(), 1);
    assert_eq!(p2_todos[0].title, "task for B");

    let no_project =
        block_on(SqliteTaskQueries::from(db.conn()).get_todos_without_project()).unwrap();
    assert_eq!(no_project.len(), 1);
    assert_eq!(no_project[0].title, "no project task");
}

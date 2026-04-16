use autograph_core::{
    Database, ProjectCommands, ProjectQueries, Queries, Repository, SqliteProjectQueries,
    SqliteProjectRepository, SqliteTaskQueries, SqliteTodoRepository, SqlxDatabase, TaskCommands,
    TaskQueries,
};

async fn fresh_db() -> SqlxDatabase {
    let db = (SqlxDatabase::open(":memory:"))
        .await
        .expect("failed to create in-memory db");
    (db.migrate()).await.expect("failed to run migrations");
    db
}

#[tokio::test]
async fn empty_database_returns_no_projects() {
    let db = fresh_db().await;
    let projects = (SqliteProjectQueries::bind(db.conn()).get_all_projects())
        .await
        .unwrap();
    assert!(projects.is_empty());
}

#[tokio::test]
async fn add_single_project() {
    let db = fresh_db().await;
    (db.transaction(async |tx| {
        let repo = SqliteProjectRepository::bind(tx);
        ProjectCommands::new(&repo).add("My Project").await
    }))
    .await
    .unwrap();
    let projects = (SqliteProjectQueries::bind(db.conn()).get_all_projects())
        .await
        .unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].title, "My Project");
}

#[tokio::test]
async fn add_multiple_projects_preserves_order() {
    let db = fresh_db().await;
    for title in &["Alpha", "Beta", "Gamma"] {
        (db.transaction(async |tx| {
            let repo = SqliteProjectRepository::bind(tx);
            ProjectCommands::new(&repo).add(title).await
        }))
        .await
        .unwrap();
    }
    let projects = (SqliteProjectQueries::bind(db.conn()).get_all_projects())
        .await
        .unwrap();
    assert_eq!(projects.len(), 3);
    assert_eq!(projects[0].title, "Alpha");
    assert_eq!(projects[1].title, "Beta");
    assert_eq!(projects[2].title, "Gamma");
}

#[tokio::test]
async fn todos_without_project_by_default() {
    let db = fresh_db().await;
    (db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo).add("inbox task").await
    }))
    .await
    .unwrap();
    let todos = (SqliteTaskQueries::bind(db.conn()).get_todos_without_project())
        .await
        .unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "inbox task");
    assert!(todos[0].project_id.is_none());
}

#[tokio::test]
async fn add_todo_with_project() {
    let db = fresh_db().await;
    let project_id = (db.transaction(async |tx| {
        let repo = SqliteProjectRepository::bind(tx);
        ProjectCommands::new(&repo).add("Work").await
    }))
    .await
    .unwrap();

    (db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo)
            .add_with_project("write report", project_id)
            .await
    }))
    .await
    .unwrap();

    let todos_by_project = (SqliteTaskQueries::bind(db.conn()).get_todos_by_project(project_id))
        .await
        .unwrap();
    assert_eq!(todos_by_project.len(), 1);
    assert_eq!(todos_by_project[0].title, "write report");
    assert_eq!(todos_by_project[0].project_id, Some(project_id));

    let todos_without = (SqliteTaskQueries::bind(db.conn()).get_todos_without_project())
        .await
        .unwrap();
    assert!(todos_without.is_empty());
}

#[tokio::test]
async fn tasks_are_filtered_by_project() {
    let db = fresh_db().await;
    let p1 = (db.transaction(async |tx| {
        let repo = SqliteProjectRepository::bind(tx);
        ProjectCommands::new(&repo).add("Project A").await
    }))
    .await
    .unwrap();
    let p2 = (db.transaction(async |tx| {
        let repo = SqliteProjectRepository::bind(tx);
        ProjectCommands::new(&repo).add("Project B").await
    }))
    .await
    .unwrap();

    (db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo)
            .add_with_project("task for A", p1)
            .await
    }))
    .await
    .unwrap();
    (db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo)
            .add_with_project("task for B", p2)
            .await
    }))
    .await
    .unwrap();
    (db.transaction(async |tx| {
        let repo = SqliteTodoRepository::bind(tx);
        TaskCommands::new(&repo).add("no project task").await
    }))
    .await
    .unwrap();

    let p1_todos = (SqliteTaskQueries::bind(db.conn()).get_todos_by_project(p1))
        .await
        .unwrap();
    assert_eq!(p1_todos.len(), 1);
    assert_eq!(p1_todos[0].title, "task for A");

    let p2_todos = (SqliteTaskQueries::bind(db.conn()).get_todos_by_project(p2))
        .await
        .unwrap();
    assert_eq!(p2_todos.len(), 1);
    assert_eq!(p2_todos[0].title, "task for B");

    let no_project = (SqliteTaskQueries::bind(db.conn()).get_todos_without_project())
        .await
        .unwrap();
    assert_eq!(no_project.len(), 1);
    assert_eq!(no_project[0].title, "no project task");
}

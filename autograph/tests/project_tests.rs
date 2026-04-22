use autograph::{
    CardCommands, CardQueries, Database, DatabaseBuilder, ProjectCommands, ProjectQueries,
    SqlxCardQueries, SqlxDatabase, SqlxDatabaseBuilder, SqlxProjectQueries,
};

async fn fresh_db() -> SqlxDatabase {
    SqlxDatabaseBuilder::open(":memory:")
        .migrate()
        .finish()
        .await
        .expect("failed to setup in-memory db")
}

#[tokio::test]
async fn empty_database_returns_no_projects() {
    let db = fresh_db().await;
    let projects = (SqlxProjectQueries::new(db.conn()).get_all_projects())
        .await
        .unwrap();
    assert!(projects.is_empty());
}

#[tokio::test]
async fn add_single_project() {
    let db = fresh_db().await;
    db.begin(async |uow| {
        ProjectCommands::new(uow).add("My Project").await?;
        Ok(())
    })
    .await
    .unwrap();
    let projects = (SqlxProjectQueries::new(db.conn()).get_all_projects())
        .await
        .unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].title, "My Project");
}

#[tokio::test]
async fn add_multiple_projects_preserves_order() {
    let db = fresh_db().await;
    for title in ["Alpha", "Beta", "Gamma"] {
        db.begin(async |uow| {
            ProjectCommands::new(uow).add(title).await?;
            Ok(())
        })
        .await
        .unwrap();
    }
    let projects = (SqlxProjectQueries::new(db.conn()).get_all_projects())
        .await
        .unwrap();
    assert_eq!(projects.len(), 3);
    assert_eq!(projects[0].title, "Alpha");
    assert_eq!(projects[1].title, "Beta");
    assert_eq!(projects[2].title, "Gamma");
}

#[tokio::test]
async fn cards_without_project_by_default() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("inbox card").await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_cards_without_project())
        .await
        .unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].title, "inbox card");
    assert!(cards[0].project_id.is_none());
}

#[tokio::test]
async fn add_card_with_project() {
    let db = fresh_db().await;
    let project = db
        .begin(async |uow| ProjectCommands::new(uow).add("Work").await)
        .await
        .unwrap();
    let project_id = project.id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .add_with_project("write report", project_id)
            .await
    })
    .await
    .unwrap();

    let cards_by_project = (SqlxCardQueries::new(db.conn()).get_cards_by_project(project_id))
        .await
        .unwrap();
    assert_eq!(cards_by_project.len(), 1);
    assert_eq!(cards_by_project[0].title, "write report");
    assert_eq!(cards_by_project[0].project_id, Some(project_id));

    let cards_without = (SqlxCardQueries::new(db.conn()).get_cards_without_project())
        .await
        .unwrap();
    assert!(cards_without.is_empty());
}

#[tokio::test]
async fn cards_are_filtered_by_project() {
    let db = fresh_db().await;
    let p1 = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project A").await)
        .await
        .unwrap()
        .id;
    let p2 = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project B").await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .add_with_project("card for A", p1)
            .await
    })
    .await
    .unwrap();
    db.begin(async |uow| {
        CardCommands::new(uow)
            .add_with_project("card for B", p2)
            .await
    })
    .await
    .unwrap();
    db.begin(async |uow| CardCommands::new(uow).add("no project card").await)
        .await
        .unwrap();

    let p1_cards = (SqlxCardQueries::new(db.conn()).get_cards_by_project(p1))
        .await
        .unwrap();
    assert_eq!(p1_cards.len(), 1);
    assert_eq!(p1_cards[0].title, "card for A");

    let p2_cards = (SqlxCardQueries::new(db.conn()).get_cards_by_project(p2))
        .await
        .unwrap();
    assert_eq!(p2_cards.len(), 1);
    assert_eq!(p2_cards[0].title, "card for B");

    let no_project = (SqlxCardQueries::new(db.conn()).get_cards_without_project())
        .await
        .unwrap();
    assert_eq!(no_project.len(), 1);
    assert_eq!(no_project[0].title, "no project card");
}

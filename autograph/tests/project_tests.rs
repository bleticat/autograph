use autograph::{
    CardCommands, CardQueries, Database, DatabaseBuilder, ProjectCommands, ProjectQueries,
    QueryFilter, SeaOrmDatabase, SeaOrmDatabaseBuilder, SectionCommands,
};
use uuid::Uuid;

const DEFAULT_LIMIT: u32 = 100;
type DatabaseAdapter = SeaOrmDatabase;
type DatabaseBuilderAdapter = SeaOrmDatabaseBuilder;

async fn fresh_db() -> DatabaseAdapter {
    DatabaseBuilderAdapter::open(":memory:")
        .migrate()
        .finish()
        .await
        .expect("failed to setup in-memory db")
}

async fn all_projects(db: &DatabaseAdapter) -> Vec<autograph::Project> {
    db.project().filter(DEFAULT_LIMIT, 0).await.unwrap()
}

async fn get_project(db: &DatabaseAdapter, project_id: Uuid) -> autograph::ProjectData {
    db.project()
        .get_project(project_id)
        .await
        .unwrap()
        .expect("project should exist")
}

async fn cards_without_project(db: &DatabaseAdapter) -> Vec<autograph::Card> {
    db.card()
        .filter(
            DEFAULT_LIMIT,
            0,
            QueryFilter::Ignore,
            QueryFilter::None,
            QueryFilter::Ignore,
        )
        .await
        .unwrap()
}

async fn cards_by_project(db: &DatabaseAdapter, project_id: Uuid) -> Vec<autograph::Card> {
    db.card()
        .filter(
            DEFAULT_LIMIT,
            0,
            QueryFilter::Ignore,
            QueryFilter::Val(project_id),
            QueryFilter::Ignore,
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn empty_database_returns_no_projects() {
    let db = fresh_db().await;
    let projects = all_projects(&db).await;
    assert!(projects.is_empty());
}

#[tokio::test]
async fn add_single_project() {
    let db = fresh_db().await;
    ProjectCommands::new(&db)
        .add("My Project")
        .await
        .unwrap();
    let projects = all_projects(&db).await;
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].title, "My Project");
}

#[tokio::test]
async fn add_multiple_projects_preserves_order() {
    let db = fresh_db().await;
    for title in ["Alpha", "Beta", "Gamma"] {
        ProjectCommands::new(&db).add(title).await.unwrap();
    }
    let projects = all_projects(&db).await;
    assert_eq!(projects.len(), 3);
    assert_eq!(projects[0].title, "Alpha");
    assert_eq!(projects[1].title, "Beta");
    assert_eq!(projects[2].title, "Gamma");
}

#[tokio::test]
async fn project_filter_respects_limit_and_offset() {
    let db = fresh_db().await;
    for title in ["Alpha", "Beta", "Gamma"] {
        ProjectCommands::new(&db).add(title).await.unwrap();
    }

    let projects = db.project().filter(1, 1).await.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].title, "Beta");
}

#[tokio::test]
async fn cards_without_project_by_default() {
    let db = fresh_db().await;
    CardCommands::new(&db).add("inbox card").await.unwrap();
    let cards = cards_without_project(&db).await;
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].title, "inbox card");
    assert!(cards[0].project_id.is_none());
}

#[tokio::test]
async fn add_card_with_project() {
    let db = fresh_db().await;
    let project_id = ProjectCommands::new(&db).add("Work").await.unwrap().id;

    CardCommands::new(&db)
        .add_with_project("write report", project_id)
        .await
        .unwrap();

    let cards_by_project = cards_by_project(&db, project_id).await;
    assert_eq!(cards_by_project.len(), 1);
    assert_eq!(cards_by_project[0].title, "write report");
    assert_eq!(cards_by_project[0].project_id, Some(project_id));

    let cards_without = cards_without_project(&db).await;
    assert!(cards_without.is_empty());
}

#[tokio::test]
async fn cards_are_filtered_by_project() {
    let db = fresh_db().await;
    let p1 = ProjectCommands::new(&db).add("Project A").await.unwrap().id;
    let p2 = ProjectCommands::new(&db).add("Project B").await.unwrap().id;

    CardCommands::new(&db)
        .add_with_project("card for A", p1)
        .await
        .unwrap();
    CardCommands::new(&db)
        .add_with_project("card for B", p2)
        .await
        .unwrap();
    CardCommands::new(&db)
        .add("no project card")
        .await
        .unwrap();

    let p1_cards = cards_by_project(&db, p1).await;
    assert_eq!(p1_cards.len(), 1);
    assert_eq!(p1_cards[0].title, "card for A");

    let p2_cards = cards_by_project(&db, p2).await;
    assert_eq!(p2_cards.len(), 1);
    assert_eq!(p2_cards[0].title, "card for B");

    let no_project = cards_without_project(&db).await;
    assert_eq!(no_project.len(), 1);
    assert_eq!(no_project[0].title, "no project card");
}

#[tokio::test]
async fn project_query_returns_sections_and_unsectioned_cards() {
    let db = fresh_db().await;
    let project_id = ProjectCommands::new(&db).add("Website").await.unwrap().id;

    let backlog_id = SectionCommands::new(&db)
        .add("Backlog", project_id)
        .await
        .unwrap()
        .id;
    SectionCommands::new(&db)
        .add("In Progress", project_id)
        .await
        .unwrap();

    CardCommands::new(&db)
        .add_with_assignment("Investigate bug", Some(project_id), None)
        .await
        .unwrap();
    CardCommands::new(&db)
        .add_with_assignment("Draft copy", Some(project_id), Some(backlog_id))
        .await
        .unwrap();

    let project = get_project(&db, project_id).await;
    assert_eq!(project.project.title, "Website");
    assert_eq!(project.sections.len(), 2);
    assert_eq!(project.sections[0].section.title, "Backlog");
    assert_eq!(project.sections[0].cards.len(), 1);
    assert_eq!(project.sections[0].cards[0].title, "Draft copy");
    assert_eq!(project.sections[1].section.title, "In Progress");
    assert!(project.sections[1].cards.is_empty());
    assert_eq!(project.cards_without_section.len(), 1);
    assert_eq!(project.cards_without_section[0].title, "Investigate bug");
}

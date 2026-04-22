use autograph::{
    Database, DatabaseBuilder, ProjectCommands, QueryFilter, SectionCommands, SectionQueries,
    SqlxDatabase, SqlxDatabaseBuilder, SqlxSectionQueries,
};
use uuid::Uuid;

const DEFAULT_LIMIT: u32 = 100;

async fn fresh_db() -> SqlxDatabase {
    SqlxDatabaseBuilder::open(":memory:")
        .migrate()
        .finish()
        .await
        .expect("failed to setup in-memory db")
}

async fn sections(db: &SqlxDatabase, project_id: QueryFilter<Uuid>) -> Vec<autograph::Section> {
    SqlxSectionQueries::new(db.conn())
        .filter(DEFAULT_LIMIT, 0, project_id)
        .await
        .unwrap()
}

#[tokio::test]
async fn sections_are_filtered_by_project() {
    let db = fresh_db().await;
    let project_a = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project A").await)
        .await
        .unwrap()
        .id;
    let project_b = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project B").await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| SectionCommands::new(uow).add("Backlog", project_a).await)
        .await
        .unwrap();
    db.begin(async |uow| {
        SectionCommands::new(uow)
            .add("In Progress", project_a)
            .await
    })
    .await
    .unwrap();
    db.begin(async |uow| SectionCommands::new(uow).add("Done", project_b).await)
        .await
        .unwrap();

    let sections_a = sections(&db, QueryFilter::Val(project_a)).await;
    assert_eq!(sections_a.len(), 2);
    assert_eq!(sections_a[0].title, "Backlog");
    assert_eq!(sections_a[1].title, "In Progress");

    let sections_b = sections(&db, QueryFilter::Val(project_b)).await;
    assert_eq!(sections_b.len(), 1);
    assert_eq!(sections_b[0].title, "Done");
}

#[tokio::test]
async fn section_filter_respects_limit_and_offset() {
    let db = fresh_db().await;
    let project_id = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project").await)
        .await
        .unwrap()
        .id;

    for title in ["Backlog", "Doing", "Done"] {
        db.begin(async |uow| SectionCommands::new(uow).add(title, project_id).await)
            .await
            .unwrap();
    }

    let sections = SqlxSectionQueries::new(db.conn())
        .filter(1, 1, QueryFilter::Val(project_id))
        .await
        .unwrap();
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].title, "Doing");
}

#[tokio::test]
async fn edit_section_updates_title() {
    let db = fresh_db().await;
    let project_id = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project").await)
        .await
        .unwrap()
        .id;
    let section_id = db
        .begin(async |uow| SectionCommands::new(uow).add("Ideas", project_id).await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| SectionCommands::new(uow).edit(section_id, "Ready").await)
        .await
        .unwrap();

    let sections = sections(&db, QueryFilter::Val(project_id)).await;
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].title, "Ready");
}

#[tokio::test]
async fn delete_section_removes_it() {
    let db = fresh_db().await;
    let project_id = db
        .begin(async |uow| ProjectCommands::new(uow).add("Project").await)
        .await
        .unwrap()
        .id;
    let section_id = db
        .begin(async |uow| SectionCommands::new(uow).add("Ideas", project_id).await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| SectionCommands::new(uow).delete(section_id).await)
        .await
        .unwrap();

    let sections = sections(&db, QueryFilter::Val(project_id)).await;
    assert!(sections.is_empty());
}

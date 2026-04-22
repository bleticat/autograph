use autograph::{
    Database, DatabaseBuilder, ProjectCommands, SectionCommands, SectionQueries, SqlxDatabase,
    SqlxDatabaseBuilder, SqlxSectionQueries,
};

async fn fresh_db() -> SqlxDatabase {
    SqlxDatabaseBuilder::open(":memory:")
        .migrate()
        .finish()
        .await
        .expect("failed to setup in-memory db")
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

    let sections_a = (SqlxSectionQueries::new(db.conn()).get_sections_by_project(project_a))
        .await
        .unwrap();
    assert_eq!(sections_a.len(), 2);
    assert_eq!(sections_a[0].title, "Backlog");
    assert_eq!(sections_a[1].title, "In Progress");

    let sections_b = (SqlxSectionQueries::new(db.conn()).get_sections_by_project(project_b))
        .await
        .unwrap();
    assert_eq!(sections_b.len(), 1);
    assert_eq!(sections_b[0].title, "Done");
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

    let sections = (SqlxSectionQueries::new(db.conn()).get_sections_by_project(project_id))
        .await
        .unwrap();
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

    let sections = (SqlxSectionQueries::new(db.conn()).get_sections_by_project(project_id))
        .await
        .unwrap();
    assert!(sections.is_empty());
}

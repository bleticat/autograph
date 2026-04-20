use chrono::NaiveDate;
use autograph::{
    Database, DatabaseBuilder, EventCommands, EventQueries, ProjectCommands, SqlxDatabase,
    SqlxDatabaseBuilder, SqlxEventQueries,
};

async fn fresh_db() -> SqlxDatabase {
    SqlxDatabaseBuilder::open(":memory:")
        .migrate()
        .finish()
        .await
        .expect("failed to setup in-memory db")
}

#[tokio::test]
async fn events_are_filtered_by_project() {
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
    let date = NaiveDate::from_ymd_opt(2026, 5, 10)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    db.begin(async |uow| {
        EventCommands::new(uow)
            .add_with_project(date, "Kickoff", "", p1)
            .await
    })
    .await
    .unwrap();
    db.begin(async |uow| {
        EventCommands::new(uow)
            .add_with_project(date, "Demo", "", p2)
            .await
    })
    .await
    .unwrap();
    db.begin(async |uow| EventCommands::new(uow).add(date, "Personal", "").await)
        .await
        .unwrap();

    let p1_events = (SqlxEventQueries::new(db.conn()).get_events_by_project(p1))
        .await
        .unwrap();
    assert_eq!(p1_events.len(), 1);
    assert_eq!(p1_events[0].title, "Kickoff");

    let p2_events = (SqlxEventQueries::new(db.conn()).get_events_by_project(p2))
        .await
        .unwrap();
    assert_eq!(p2_events.len(), 1);
    assert_eq!(p2_events[0].title, "Demo");

    let no_project = (SqlxEventQueries::new(db.conn()).get_events_without_project())
        .await
        .unwrap();
    assert_eq!(no_project.len(), 1);
    assert_eq!(no_project[0].title, "Personal");
}

#[tokio::test]
async fn edit_event_updates_fields() {
    let db = fresh_db().await;
    let date = NaiveDate::from_ymd_opt(2026, 5, 10)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    db.begin(async |uow| EventCommands::new(uow).add(date, "Kickoff", "").await)
        .await
        .unwrap();
    let event_id = (SqlxEventQueries::new(db.conn()).get_all_events())
        .await
        .unwrap()[0]
        .id;

    let updated_date = NaiveDate::from_ymd_opt(2026, 5, 11)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    db.begin(async |uow| {
        EventCommands::new(uow)
            .edit(
                event_id,
                updated_date,
                "Updated kickoff",
                "Agenda confirmed",
            )
            .await
    })
    .await
    .unwrap();

    let event = (SqlxEventQueries::new(db.conn()).get_all_events())
        .await
        .unwrap()[0]
        .clone();
    assert_eq!(event.title, "Updated kickoff");
    assert_eq!(event.description, "Agenda confirmed");
    assert_eq!(
        event.date.date_naive(),
        NaiveDate::from_ymd_opt(2026, 5, 11).unwrap()
    );
}

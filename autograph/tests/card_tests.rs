use autograph::{
    CardCommands, CardQueries, Database, DatabaseBuilder, ProjectCommands, SectionCommands,
    SqlxCardQueries, SqlxDatabase, SqlxDatabaseBuilder,
};
use chrono::NaiveDate;
use uuid::Uuid;

async fn fresh_db() -> SqlxDatabase {
    SqlxDatabaseBuilder::open(":memory:")
        .migrate()
        .finish()
        .await
        .expect("failed to setup in-memory db")
}

#[tokio::test]
async fn empty_database_returns_no_cards() {
    let db = fresh_db().await;
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert!(cards.is_empty());
}

#[tokio::test]
async fn add_single_card() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("buy milk").await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].title, "buy milk");
    assert_eq!(cards[0].description, "");
    assert_eq!(cards[0].deadline, None);
    assert!(!cards[0].completed);
    assert_eq!(cards[0].project_id, None);
    assert_eq!(cards[0].section_id, None);
}

#[tokio::test]
async fn add_multiple_cards_preserves_order() {
    let db = fresh_db().await;
    for title in ["first", "second", "third"] {
        db.begin(async |uow| CardCommands::new(uow).add(title).await)
            .await
            .unwrap();
    }
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert_eq!(cards.len(), 3);
    assert_eq!(cards[0].title, "first");
    assert_eq!(cards[1].title, "second");
    assert_eq!(cards[2].title, "third");
}

#[tokio::test]
async fn toggle_marks_completed() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("card").await)
        .await
        .unwrap();
    let id = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| CardCommands::new(uow).toggle(id).await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert!(cards[0].completed);
}

#[tokio::test]
async fn toggle_twice_restores_incomplete() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("card").await)
        .await
        .unwrap();
    let id = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| CardCommands::new(uow).toggle(id).await)
        .await
        .unwrap();
    db.begin(async |uow| CardCommands::new(uow).toggle(id).await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert!(!cards[0].completed);
}

#[tokio::test]
async fn delete_removes_card() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("to delete").await)
        .await
        .unwrap();
    let id = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| CardCommands::new(uow).delete(id).await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert!(cards.is_empty());
}

#[tokio::test]
async fn delete_only_target_card() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("keep").await)
        .await
        .unwrap();
    db.begin(async |uow| CardCommands::new(uow).add("remove").await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    let remove_id = cards[1].id;

    db.begin(async |uow| CardCommands::new(uow).delete(remove_id).await)
        .await
        .unwrap();
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].title, "keep");
}

#[tokio::test]
async fn toggle_nonexistent_id_is_noop() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).toggle(Uuid::new_v4()).await)
        .await
        .unwrap();
    assert!(
        (SqlxCardQueries::new(db.conn()).get_all_cards())
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn delete_nonexistent_id_is_noop() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("still here").await)
        .await
        .unwrap();
    db.begin(async |uow| CardCommands::new(uow).delete(Uuid::new_v4()).await)
        .await
        .unwrap();
    assert_eq!(
        (SqlxCardQueries::new(db.conn()).get_all_cards())
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn edit_updates_card_fields() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("draft").await)
        .await
        .unwrap();
    let id = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .edit(
                id,
                "final title",
                "expanded card details",
                Some(
                    NaiveDate::from_ymd_opt(2026, 5, 10)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                        .and_utc(),
                ),
                None,
                None,
            )
            .await
    })
    .await
    .unwrap();

    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert_eq!(cards[0].title, "final title");
    assert_eq!(cards[0].description, "expanded card details");
    assert_eq!(
        cards[0].deadline.map(|deadline| deadline.date_naive()),
        Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap())
    );
    assert_eq!(cards[0].project_id, None);
    assert_eq!(cards[0].section_id, None);
}

#[tokio::test]
async fn add_card_with_section_assigns_project_and_section() {
    let db = fresh_db().await;
    let project_id = db
        .begin(async |uow| ProjectCommands::new(uow).add("Work").await)
        .await
        .unwrap()
        .id;
    let section_id = db
        .begin(async |uow| SectionCommands::new(uow).add("Today", project_id).await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .add_with_section("fix bug", Some(project_id), section_id)
            .await
    })
    .await
    .unwrap();

    let cards = (SqlxCardQueries::new(db.conn()).get_cards_by_project(project_id))
        .await
        .unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].title, "fix bug");
    assert_eq!(cards[0].project_id, Some(project_id));
    assert_eq!(cards[0].section_id, Some(section_id));
}

#[tokio::test]
async fn add_card_rejects_section_from_another_project() {
    let db = fresh_db().await;
    let project_a = db
        .begin(async |uow| ProjectCommands::new(uow).add("A").await)
        .await
        .unwrap()
        .id;
    let project_b = db
        .begin(async |uow| ProjectCommands::new(uow).add("B").await)
        .await
        .unwrap()
        .id;
    let section_id = db
        .begin(async |uow| SectionCommands::new(uow).add("Doing", project_a).await)
        .await
        .unwrap()
        .id;

    let err = db
        .begin(async |uow| {
            CardCommands::new(uow)
                .add_with_section("wrong place", Some(project_b), section_id)
                .await
        })
        .await
        .unwrap_err();

    assert!(
        err.to_string()
            .contains("Section must belong to the selected project")
    );
}

#[tokio::test]
async fn edit_card_can_move_between_project_and_section() {
    let db = fresh_db().await;
    let project_id = db
        .begin(async |uow| ProjectCommands::new(uow).add("Work").await)
        .await
        .unwrap()
        .id;
    let section_id = db
        .begin(async |uow| SectionCommands::new(uow).add("Later", project_id).await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .add_with_project("prepare notes", project_id)
            .await
    })
    .await
    .unwrap();
    let card_id = (SqlxCardQueries::new(db.conn()).get_cards_by_project(project_id))
        .await
        .unwrap()[0]
        .id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .edit(
                card_id,
                "prepare notes",
                "",
                None,
                Some(project_id),
                Some(section_id),
            )
            .await
    })
    .await
    .unwrap();

    let card = (SqlxCardQueries::new(db.conn()).get_cards_by_project(project_id))
        .await
        .unwrap()[0]
        .clone();
    assert_eq!(card.section_id, Some(section_id));

    db.begin(async |uow| {
        CardCommands::new(uow)
            .edit(card_id, "prepare notes", "", None, Some(project_id), None)
            .await
    })
    .await
    .unwrap();

    let card = (SqlxCardQueries::new(db.conn()).get_cards_by_project(project_id))
        .await
        .unwrap()[0]
        .clone();
    assert_eq!(card.project_id, Some(project_id));
    assert_eq!(card.section_id, None);
}

#[tokio::test]
async fn deleting_section_keeps_cards_in_project() {
    let db = fresh_db().await;
    let project_id = db
        .begin(async |uow| ProjectCommands::new(uow).add("Work").await)
        .await
        .unwrap()
        .id;
    let section_id = db
        .begin(async |uow| SectionCommands::new(uow).add("Soon", project_id).await)
        .await
        .unwrap()
        .id;

    db.begin(async |uow| {
        CardCommands::new(uow)
            .add_with_section("ship release", Some(project_id), section_id)
            .await
    })
    .await
    .unwrap();

    db.begin(async |uow| SectionCommands::new(uow).delete(section_id).await)
        .await
        .unwrap();

    let cards = (SqlxCardQueries::new(db.conn()).get_cards_by_project(project_id))
        .await
        .unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].project_id, Some(project_id));
    assert_eq!(cards[0].section_id, None);
}

#[tokio::test]
async fn ids_are_unique_after_delete() {
    let db = fresh_db().await;
    db.begin(async |uow| CardCommands::new(uow).add("first").await)
        .await
        .unwrap();
    let first_id = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap()[0]
        .id;
    db.begin(async |uow| CardCommands::new(uow).delete(first_id).await)
        .await
        .unwrap();

    db.begin(async |uow| CardCommands::new(uow).add("second").await)
        .await
        .unwrap();
    let second_id = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap()[0]
        .id;
    assert_ne!(first_id, second_id);
}

#[tokio::test]
async fn full_workflow() {
    let db = fresh_db().await;

    // Add a few cards
    for title in ["buy groceries", "write tests", "deploy app"] {
        db.begin(async |uow| CardCommands::new(uow).add(title).await)
            .await
            .unwrap();
    }

    // Complete one
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    let middle_card_id = cards[1].id;
    let last_card_id = cards[2].id;
    db.begin(async |uow| CardCommands::new(uow).toggle(middle_card_id).await)
        .await
        .unwrap();

    // Delete one
    db.begin(async |uow| CardCommands::new(uow).delete(last_card_id).await)
        .await
        .unwrap();

    // Verify final state
    let cards = (SqlxCardQueries::new(db.conn()).get_all_cards())
        .await
        .unwrap();
    assert_eq!(cards.len(), 2);
    assert_eq!(cards[0].title, "buy groceries");
    assert_eq!(cards[0].description, "");
    assert_eq!(cards[0].deadline, None);
    assert!(!cards[0].completed);
    assert_eq!(cards[1].title, "write tests");
    assert_eq!(cards[1].description, "");
    assert_eq!(cards[1].deadline, None);
    assert!(cards[1].completed);
}

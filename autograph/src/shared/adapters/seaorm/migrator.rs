mod m20260423_000001_create_schema;
mod m20260424_000002_event_source_cards;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260423_000001_create_schema::Migration),
            Box::new(m20260424_000002_event_source_cards::Migration),
        ]
    }
}

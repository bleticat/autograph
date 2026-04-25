use crate::card::entity::Card;
use crate::shared::error::AppErr;
use crate::shared::filter::QueryFilter;
use chrono::{DateTime, Utc};
use std::future::Future;
use uuid::Uuid;

pub trait CardQueries {
    fn filter(
        &self,
        limit: u32,
        offset: u32,
        deadline: QueryFilter<DateTime<Utc>>,
        project_id: QueryFilter<Uuid>,
        section_id: QueryFilter<Uuid>,
    ) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send;

    fn get_all_cards(&self) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send {
        self.filter(
            u32::MAX,
            0,
            QueryFilter::Ignore,
            QueryFilter::Ignore,
            QueryFilter::Ignore,
        )
    }

    fn get_cards_without_project(&self) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send {
        self.filter(
            u32::MAX,
            0,
            QueryFilter::Ignore,
            QueryFilter::None,
            QueryFilter::Ignore,
        )
    }

    fn get_cards_by_project(
        &self,
        project_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Card>, AppErr>> + Send {
        self.filter(
            u32::MAX,
            0,
            QueryFilter::Ignore,
            QueryFilter::Val(project_id),
            QueryFilter::Ignore,
        )
    }
}

use crate::section::entity::Section;
use crate::section::queries::SectionQueries;
use crate::shared::adapters::seaorm::database::SeaOrmConnection;
use crate::shared::adapters::seaorm::models::section as section_model;
use crate::shared::error::AppErr;
use crate::shared::filter::QueryFilter;
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter as SeaOrmQueryFilter, QueryOrder, QuerySelect,
    sea_query::Expr,
};
use uuid::Uuid;

pub struct SeaOrmSectionQueries {
    conn: SeaOrmConnection,
}

impl SeaOrmSectionQueries {
    pub fn new(conn: SeaOrmConnection) -> Self {
        Self { conn }
    }
}

impl SectionQueries for SeaOrmSectionQueries {
    async fn filter(
        &self,
        limit: u32,
        offset: u32,
        project_id: QueryFilter<Uuid>,
    ) -> Result<Vec<Section>, AppErr> {
        let mut query = section_model::Entity::find()
            .order_by_asc(Expr::cust("rowid"))
            .limit(u64::from(limit))
            .offset(u64::from(offset));

        query = match project_id {
            QueryFilter::Val(project_id) => {
                query.filter(section_model::Column::ProjectId.eq(project_id))
            }
            QueryFilter::None => query.filter(section_model::Column::ProjectId.is_null()),
            QueryFilter::Ignore => query,
        };

        let sections = query.all(&self.conn).await?;
        Ok(sections.into_iter().map(to_section).collect())
    }
}

fn to_section(model: section_model::Model) -> Section {
    Section {
        id: model.id,
        title: model.title,
        project_id: model.project_id,
    }
}

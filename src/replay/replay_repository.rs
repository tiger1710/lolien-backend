use async_trait::async_trait;

use entity::match_info::{self, Entity as MatchInfo};
use entity::participant::{self, Entity as Participant};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, InsertResult, QueryFilter};

pub struct ReplayRepositoryImpl {
    conn: DatabaseConnection,
}

impl ReplayRepositoryImpl {
    pub const fn new(conn: DatabaseConnection) -> ReplayRepositoryImpl {
        ReplayRepositoryImpl { conn }
    }
}

#[async_trait]
impl super::ReplayRepository for ReplayRepositoryImpl {
    async fn insert_match_info(
        &self,
        match_info: match_info::ActiveModel,
    ) -> anyhow::Result<InsertResult<match_info::ActiveModel>> {
        Ok(MatchInfo::insert(match_info).exec(&self.conn).await?)
    }

    async fn select_match_info(
        &self,
        match_id: &str,
    ) -> anyhow::Result<Option<entity::match_info::Model>> {
        Ok(MatchInfo::find_by_id(match_id).one(&self.conn).await?)
    }

    async fn insert_many_participants(
        &self,
        participants: Vec<participant::ActiveModel>,
    ) -> anyhow::Result<InsertResult<participant::ActiveModel>> {
        Ok(Participant::insert_many(participants)
            .exec(&self.conn)
            .await?)
    }

    async fn select_participants_by_match_id(
        &self,
        match_id: &str,
    ) -> anyhow::Result<Vec<entity::participant::Model>> {
        Ok(Participant::find()
            .filter(participant::Column::MatchId.eq(match_id))
            .all(&self.conn)
            .await?)
    }
}

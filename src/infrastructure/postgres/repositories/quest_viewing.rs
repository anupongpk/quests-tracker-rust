use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::quests::QuestEntity, repositories::quest_viewing::QuestViewingRepository,
        value_objecs::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct QuestViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestViewingRepository for QuestViewingPostgres {
    async fn view_detail(&self, quest_id: i32) -> Result<QuestEntity> {
        unimplemented!()
    }
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        unimplemented!()
    }
    async fn advanturers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        unimplemented!()
    }
}

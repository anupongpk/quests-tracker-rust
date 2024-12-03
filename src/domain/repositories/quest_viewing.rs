use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::{
    entities::quests::QuestEntity, value_objecs::board_checking_filter::BoardCheckingFilter,
};

#[async_trait]
#[automock]

pub trait QuestViewingRepository {
    async fn view_detail(&self, quest_id: i32) -> Result<QuestEntity>;
    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>>;
    async fn advanturers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64>;
}
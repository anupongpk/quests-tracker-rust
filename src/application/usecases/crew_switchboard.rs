use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::{
    crew_switchboard::{self, CrewSwitchBoardRepository},
    quest_viewing::QuestViewingRepository,
};

use super::quest_viewing;

pub struct CrewSwitcBoardUseCase<T1, T2>
where
    T1: CrewSwitchBoardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    crew_switchboard_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> CrewSwitcBoardUseCase<T1, T2>
where
    T1: CrewSwitchBoardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(crew_switchboard_repository: Arc<T1>, quest_viewing_repository: Arc<T2>) -> Self {
        Self {
            crew_switchboard_repository,
            quest_viewing_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        unimplemented!()
    }

    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        unimplemented!()
    }
}

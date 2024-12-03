use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchBoardRepository,
        value_objecs::quest_adventurer_junction::QuestAdventurerJunction,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct CrewSwitcBoardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitcBoardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
#[automock]
impl CrewSwitchBoardRepository for CrewSwitcBoardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!()
    }
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        unimplemented!()
    }
}

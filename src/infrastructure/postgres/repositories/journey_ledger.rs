use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::repositories::journey_ledger::JourneyLedgerRepoitory,
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct JourneyLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl JourneyLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl JourneyLedgerRepoitory for JourneyLedgerPostgres {
    async fn in_jourey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
    async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
    async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
}

use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::adventurers::AdventurersRepository,
    value_objecs::adventuere_model::RegisterAdventurerModel,
};

pub struct AdventuresUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventuresUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        unimplemented!()
    }
}

use serde::{Deserialize, Serialize};

use super::quest_statuses::QuestStatues;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardCheckingFilter {
    pub name: Option<String>,
    pub status: Option<QuestStatues>,
}

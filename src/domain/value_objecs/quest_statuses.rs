use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum QuestStatues {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestStatues::Open => write!(f, "Open"),
            QuestStatues::InJourney => write!(f, "InJourney"),
            QuestStatues::Completed => write!(f, "Completed"),
            QuestStatues::Failed => write!(f, "Failed"),
        }
    }
}

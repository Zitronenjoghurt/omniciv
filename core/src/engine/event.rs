use crate::types::building::Building;
use crate::types::milestone::Milestone;
use std::fmt::Display;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Event {
    UnlockedBuilding(Building),
    UnlockedMilestone(Milestone),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnlockedBuilding(building) => write!(f, "Unlocked building '{building}'!"),
            Self::UnlockedMilestone(milestone) => write!(f, "Milestone: {milestone}"),
        }
    }
}

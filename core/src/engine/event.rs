use crate::types::building::Building;
use crate::types::human::Human;
use crate::types::milestone::Milestone;
use crate::types::technology::Technology;
use std::fmt::Display;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Event {
    ResearchedTechnology(Technology),
    UnlockedBuilding(Building),
    UnlockedHuman(Human),
    UnlockedMilestone(Milestone),
    UnlockedTechnology(Technology),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResearchedTechnology(technology) => {
                write!(f, "Researched technology '{technology}'!")
            }
            Self::UnlockedBuilding(building) => write!(f, "Unlocked building '{building}'!"),
            Self::UnlockedHuman(human) => write!(f, "Unlocked new role '{human}'!"),
            Self::UnlockedMilestone(milestone) => write!(f, "Milestone: {milestone}"),
            Self::UnlockedTechnology(technology) => {
                write!(f, "Unlocked technology '{technology}'!")
            }
        }
    }
}

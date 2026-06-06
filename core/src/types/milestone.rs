use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;
use crate::types::stat::Stat;
use crate::{modifiers, Resource};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Milestone {
    BerryGathering = 0,
}

impl Milestone {
    pub fn unlock_condition(&self) -> Condition {
        match self {
            Self::BerryGathering => Condition::Always,
        }
    }

    pub fn modifiers(&self) -> &'static [Modifier] {
        match self {
            Self::BerryGathering => modifiers![
                Stat::ResourceGather(Resource::Berries) => +Value::amount(1.0);
            ],
        }
    }

    pub fn on_unlock(&self) -> Option<Action> {
        match self {
            Self::BerryGathering => None,
        }
    }
}

impl Display for Milestone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BerryGathering => write!(
                f,
                "You have learnt that berries are more than just fun to squash."
            ),
        }
    }
}

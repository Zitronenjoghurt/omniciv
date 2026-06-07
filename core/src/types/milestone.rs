use crate::engine::capabilities::auto_unlockable::AutoUnlockable;
use crate::engine::capabilities::modifying::Modifying;
use crate::engine::capabilities::type_count::TypeCount;
use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;
use crate::engine::event::Event;
use crate::types::stat::Stat;
use crate::{config, modifiers, Resource};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Milestone {
    BerryGathering = 0,
}

impl TypeCount for Milestone {
    fn count() -> usize {
        Self::COUNT
    }
}

impl TypeIter for Milestone {
    fn iter_all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

impl AutoUnlockable for Milestone {
    fn unlock_count() -> Value {
        Value::MilestonesUnlocked
    }

    fn is_unlocked(&self) -> Condition {
        Condition::MilestoneUnlocked(*self)
    }

    fn can_unlock(&self) -> Condition {
        match self {
            Self::BerryGathering => Condition::Always,
        }
    }

    fn unlock_action(&self) -> Action {
        Action::UnlockMilestone(*self)
    }

    fn unlock_event(&self) -> Option<Event> {
        Some(Event::UnlockedMilestone(*self))
    }

    fn on_unlock(&self) -> Option<Action> {
        match self {
            Self::BerryGathering => None,
        }
    }
}

impl Modifying for Milestone {
    fn modifying_active(&self) -> Condition {
        self.is_unlocked()
    }

    fn modifying_scale(&self) -> Value {
        1.0.into()
    }

    fn modifiers(&self) -> &'static [Modifier] {
        match self {
            Self::BerryGathering => modifiers![
                Stat::ResourceGather(Resource::Berries) => +Value::amount(config::gather::BERRIES_PER_GATHER);
            ],
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

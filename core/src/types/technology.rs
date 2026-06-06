use crate::engine::capabilities::auto_unlockable::AutoUnlockable;
use crate::engine::capabilities::type_count::TypeCount;
use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::condition::Condition::TechnologyUnlocked;
use crate::engine::dsl::value::Value;
use crate::engine::event::Event;
use crate::types::human::Human;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Technology {
    Fire = 0,
}

impl TypeCount for Technology {
    fn count() -> usize {
        Self::COUNT
    }
}

impl TypeIter for Technology {
    fn iter_all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

impl AutoUnlockable for Technology {
    fn unlock_count() -> Value {
        Value::TechnologiesUnlocked
    }

    fn is_unlocked(&self) -> Condition {
        TechnologyUnlocked(*self)
    }

    fn can_unlock(&self) -> Condition {
        match self {
            Self::Fire => Value::HumanCount(Human::Thinker).at_least(2),
        }
    }

    fn unlock_action(&self) -> Action {
        Action::UnlockTechnology(*self)
    }

    fn unlock_event(&self) -> Event {
        Event::UnlockedTechnology(*self)
    }

    fn on_unlock(&self) -> Option<Action> {
        None
    }
}

impl Display for Technology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fire => write!(f, "Fire"),
        }
    }
}

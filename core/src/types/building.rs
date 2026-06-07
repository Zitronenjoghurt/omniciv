use crate::engine::capabilities::auto_unlockable::AutoUnlockable;
use crate::engine::capabilities::individual_count::IndividualCount;
use crate::engine::capabilities::modifying::Modifying;
use crate::engine::capabilities::type_count::TypeCount;
use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::cost::Cost;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;
use crate::engine::event::Event;
use crate::modifiers;
use crate::types::stat::Stat;
use crate::types::technology::Technology;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Building {
    Bonfire = 0,
}

impl TypeCount for Building {
    fn count() -> usize {
        Self::COUNT
    }
}

impl TypeIter for Building {
    fn iter_all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

impl AutoUnlockable for Building {
    fn unlock_count() -> Value {
        Value::BuildingsUnlocked
    }

    fn is_unlocked(&self) -> Condition {
        Condition::BuildingUnlocked(*self)
    }

    fn can_unlock(&self) -> Condition {
        match self {
            Self::Bonfire => Condition::TechnologyResearched(Technology::Fire),
        }
    }

    fn unlock_action(&self) -> Action {
        Action::UnlockBuilding(*self)
    }

    fn unlock_event(&self) -> Option<Event> {
        Some(Event::UnlockedBuilding(*self))
    }

    fn on_unlock(&self) -> Option<Action> {
        None
    }
}

impl Modifying for Building {
    fn modifying_active(&self) -> Condition {
        self.is_unlocked()
    }

    fn modifying_scale(&self) -> Value {
        Value::BuildingCount(*self)
    }

    fn modifiers(&self) -> &'static [Modifier] {
        match self {
            // TODO: give the Bonfire a real effect
            Self::Bonfire => modifiers!(),
        }
    }
}

impl IndividualCount for Building {
    fn total_count_stat() -> Stat {
        Stat::TotalBuildings
    }

    fn individual_count(&self) -> Value {
        Value::BuildingCount(*self)
    }
}

impl Building {
    pub fn build_cost(&self) -> Cost<'_> {
        match self {
            Self::Bonfire => Cost::EMPTY,
        }
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bonfire => write!(f, "Bonfire"),
        }
    }
}

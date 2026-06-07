use crate::engine::capabilities::auto_unlockable::AutoUnlockable;
use crate::engine::capabilities::individual_count::IndividualCount;
use crate::engine::capabilities::modifying::Modifying;
use crate::engine::capabilities::type_count::TypeCount;
use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;
use crate::engine::event::Event;
use crate::types::stat::Stat;
use crate::{modifiers, Resource};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Human {
    Gatherer = 0,
    Thinker = 1,
}

impl TypeCount for Human {
    fn count() -> usize {
        Self::COUNT
    }
}

impl TypeIter for Human {
    fn iter_all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}

impl AutoUnlockable for Human {
    fn unlock_count() -> Value {
        Value::HumansUnlocked
    }

    fn is_unlocked(&self) -> Condition {
        Condition::HumanUnlocked(*self)
    }

    fn can_unlock(&self) -> Condition {
        match self {
            Self::Gatherer => Condition::Always,
            Self::Thinker => Value::Stat(Stat::ResourceNet(Resource::Berries)).at_least(0.05),
        }
    }

    fn unlock_action(&self) -> Action {
        Action::UnlockHuman(*self)
    }

    fn unlock_event(&self) -> Event {
        Event::UnlockedHuman(*self)
    }

    fn on_unlock(&self) -> Option<Action> {
        None
    }
}

impl Modifying for Human {
    fn modifying_active(&self) -> Condition {
        self.is_unlocked()
    }

    fn modifying_scale(&self) -> Value {
        Value::HumanCount(*self)
    }

    fn modifiers(&self) -> &'static [Modifier] {
        match self {
            Self::Gatherer => modifiers!(
                Stat::ResourceProduction(Resource::Berries) => +Value::amount(0.035);
                Stat::ResourceConsumption(Resource::Berries) => +Value::amount(0.03);
            ),
            Self::Thinker => modifiers!(
                Stat::ResourceProduction(Resource::Ideas) => +Value::amount(0.00003);
                Stat::ResourceConsumption(Resource::Berries) => +Value::amount(0.03);
            ),
        }
    }
}

impl IndividualCount for Human {
    fn total_count_stat() -> Stat {
        Stat::TotalHumans
    }

    fn individual_count(&self) -> Value {
        Value::HumanCount(*self)
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gatherer => write!(f, "Gatherer"),
            Self::Thinker => write!(f, "Thinker"),
        }
    }
}

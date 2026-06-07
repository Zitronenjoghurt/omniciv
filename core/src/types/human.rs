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
use crate::types::stat::Stat;
use crate::{config, modifiers, Resource};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Human {
    Idle = 0,
    Gatherer = 1,
    Thinker = 2,
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
            Self::Gatherer | Self::Idle => Condition::Always,
            Self::Thinker => Value::Stat(Stat::ResourceNet(Resource::Berries))
                .at_least(config::human::THINKER_UNLOCK_BERRY_NET.per_second()),
        }
    }

    fn unlock_action(&self) -> Action {
        Action::UnlockHuman(*self)
    }

    fn unlock_event(&self) -> Option<Event> {
        if matches!(self, Self::Idle) {
            None
        } else {
            Some(Event::UnlockedHuman(*self))
        }
    }

    fn on_unlock(&self) -> Option<Action> {
        match self {
            Self::Idle => Some(Action::AddHumans(
                Self::Idle,
                config::human::STARTING_IDLE_HUMANS,
            )),
            _ => None,
        }
    }
}

const HUMAN_EFFICIENCY: Value = Value::Stat(Stat::HumanProductionEfficiency);
const IDLE_SCALE: Value = Value::Mul(&Value::HumanCount(Human::Idle), &HUMAN_EFFICIENCY);
const GATHERER_SCALE: Value = Value::Mul(&Value::HumanCount(Human::Gatherer), &HUMAN_EFFICIENCY);
const THINKER_SCALE: Value = Value::Mul(&Value::HumanCount(Human::Thinker), &HUMAN_EFFICIENCY);

impl Modifying for Human {
    fn modifying_active(&self) -> Condition {
        self.is_unlocked()
    }

    fn modifying_scale(&self) -> Value {
        match self {
            Self::Idle => IDLE_SCALE,
            Self::Gatherer => GATHERER_SCALE,
            Self::Thinker => THINKER_SCALE,
        }
    }

    fn modifiers(&self) -> &'static [Modifier] {
        match self {
            Self::Idle => modifiers!(
                Stat::ResourceConsumption(Resource::Berries) => +Value::amount(config::human::IDLE_BERRY_UPKEEP.per_second());
            ),
            Self::Gatherer => modifiers!(
                Stat::ResourceProduction(Resource::Berries) => +Value::amount(config::human::GATHERER_BERRY_PRODUCTION.per_second());
                Stat::ResourceConsumption(Resource::Berries) => +Value::amount(config::human::GATHERER_BERRY_UPKEEP.per_second());
            ),
            Self::Thinker => modifiers!(
                Stat::ResourceProduction(Resource::Ideas) => +Value::amount(config::human::THINKER_IDEA_PRODUCTION.per_second());
                Stat::ResourceConsumption(Resource::Berries) => +Value::amount(config::human::THINKER_BERRY_UPKEEP.per_second());
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

const GROW_COST: Value = Value::Scale {
    base: &Value::amount(config::human::GROW_BASE_COST),
    factor: config::human::GROW_COST_RATIO,
    exponent: &Value::Stat(Stat::TotalHumans),
};

impl Human {
    pub fn grow_cost() -> Cost<'static> {
        const COST: Cost = Cost::new(&[(Resource::Berries, GROW_COST)]);
        COST
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gatherer => write!(f, "Gatherer"),
            Self::Thinker => write!(f, "Thinker"),
            Self::Idle => write!(f, "Idle"),
        }
    }
}

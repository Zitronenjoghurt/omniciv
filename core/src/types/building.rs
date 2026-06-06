use crate::engine::dsl::comp::CompOp;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::constant::Constant;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;
use crate::modifiers;
use crate::types::resource::Resource;
use crate::types::stat::Stat;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Building {
    Bush = 0,
}

impl Building {
    pub fn unlock_condition(&self) -> Condition {
        match self {
            Self::Bush => Value::ResourceAmount(Resource::Berries).at_least(10.0),
        }
    }

    pub fn modifiers(&self) -> &'static [Modifier] {
        match self {
            Self::Bush => modifiers!(
                Stat::ResourceGather(Resource::Berries) => +Value::amount(1.0);
            ),
        }
    }

    pub fn build_cost(&self) -> &'static [(Resource, f64)] {
        match self {
            Self::Bush => &[(Resource::Berries, 10.0)],
        }
    }
}

impl Building {
    pub fn affordable(&self, count: u128) -> Condition {
        Condition::All(
            self.build_cost()
                .iter()
                .map(|&(resource, amount)| {
                    Condition::Compare(
                        Value::ResourceAmount(resource),
                        CompOp::Gte,
                        Value::Constant(Constant::Amount(amount * count as f64)),
                    )
                })
                .collect(),
        )
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bush => write!(f, "Bush"),
        }
    }
}

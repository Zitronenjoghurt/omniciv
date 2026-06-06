use crate::engine::dsl::comp::CompOp;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::constant::Constant;
use crate::state::State;
use crate::types::building::Building;
use crate::types::resource::Resource;
use crate::types::stat::Stat;
use std::fmt::Display;

pub enum Value {
    Constant(Constant),
    BuildingCount(Building),
    BuildingsUnlocked,
    MilestonesUnlocked,
    ResourceAmount(Resource),
    ResourceGatherAmount(Resource),
}

impl Value {
    pub fn resolve(&self, state: &State) -> Constant {
        match self {
            Self::Constant(constant) => *constant,
            Self::BuildingCount(building) => Constant::Count(state.buildings.get(building)),
            Self::BuildingsUnlocked => Constant::Count(state.building_unlocks.count_set() as u128),
            Self::MilestonesUnlocked => Constant::Count(state.milestones.count_set() as u128),
            Self::ResourceAmount(resource) => Constant::Amount(state.resources.get(resource)),
            Self::ResourceGatherAmount(resource) => {
                Constant::Amount(state.stats.get(Stat::ResourceGather(*resource)))
            }
        }
    }

    pub fn equal(self, rhs: impl Into<Value>) -> Condition {
        Condition::Compare(self, CompOp::Eq, rhs.into())
    }

    pub fn not_equal(self, rhs: impl Into<Value>) -> Condition {
        Condition::Compare(self, CompOp::Neq, rhs.into())
    }

    pub fn more_than(self, rhs: impl Into<Value>) -> Condition {
        Condition::Compare(self, CompOp::Gt, rhs.into())
    }

    pub fn at_least(self, rhs: impl Into<Value>) -> Condition {
        Condition::Compare(self, CompOp::Gte, rhs.into())
    }

    pub fn less_than(self, rhs: impl Into<Value>) -> Condition {
        Condition::Compare(self, CompOp::Lt, rhs.into())
    }

    pub fn at_most(self, rhs: impl Into<Value>) -> Condition {
        Condition::Compare(self, CompOp::Lte, rhs.into())
    }

    pub const fn amount(amount: f64) -> Self {
        Self::Constant(Constant::Amount(amount))
    }

    pub const fn count(amount: u128) -> Self {
        Self::Constant(Constant::Count(amount))
    }
}

impl From<Constant> for Value {
    fn from(constant: Constant) -> Self {
        Self::Constant(constant)
    }
}

impl From<f64> for Value {
    fn from(amount: f64) -> Self {
        Self::Constant(Constant::Amount(amount))
    }
}

impl From<u128> for Value {
    fn from(count: u128) -> Self {
        Self::Constant(Constant::Count(count))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(constant) => write!(f, "{constant}"),
            Self::BuildingCount(building) => write!(f, "number of building '{building}'"),
            Self::BuildingsUnlocked => write!(f, "number of buildings unlocked"),
            Self::MilestonesUnlocked => write!(f, "number of milestones unlocked"),
            Self::ResourceAmount(resource) => write!(f, "amount of resource '{resource}'"),
            Self::ResourceGatherAmount(resource) => {
                write!(f, "gatherable amount of resource '{resource}'")
            }
        }
    }
}

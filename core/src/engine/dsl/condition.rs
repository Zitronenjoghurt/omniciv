use crate::engine::dsl::comp::CompOp;
use crate::engine::dsl::value::Value;
use crate::fmt::fmt_join;
use crate::state::State;
use crate::types::building::Building;
use crate::types::milestone::Milestone;
use std::fmt::Display;
use std::ops::{BitAnd, BitOr, Not};

pub enum Condition {
    Always,
    Never,
    BuildingUnlocked(Building),
    MilestoneUnlocked(Milestone),
    Compare(Value, CompOp, Value),
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn resolve(&self, state: &State) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::BuildingUnlocked(building) => state.building_unlocks.get(building),
            Self::MilestoneUnlocked(milestone) => state.milestones.get(milestone),
            Self::Compare(lhs, op, rhs) => {
                let lhs = lhs.resolve(state);
                let rhs = rhs.resolve(state);
                match op {
                    CompOp::Eq => lhs == rhs,
                    CompOp::Neq => lhs != rhs,
                    CompOp::Gt => lhs > rhs,
                    CompOp::Gte => lhs >= rhs,
                    CompOp::Lt => lhs < rhs,
                    CompOp::Lte => lhs <= rhs,
                }
            }
            Self::All(conditions) => conditions.iter().all(|condition| condition.resolve(state)),
            Self::Any(conditions) => conditions.iter().any(|condition| condition.resolve(state)),
            Self::Not(condition) => !condition.resolve(state),
        }
    }
}

impl BitAnd for Condition {
    type Output = Condition;
    fn bitand(self, rhs: Condition) -> Condition {
        match self {
            Condition::All(mut v) => {
                v.push(rhs);
                Condition::All(v)
            }
            lhs => Condition::All(vec![lhs, rhs]),
        }
    }
}

impl BitOr for Condition {
    type Output = Condition;
    fn bitor(self, rhs: Condition) -> Condition {
        match self {
            Condition::Any(mut v) => {
                v.push(rhs);
                Condition::Any(v)
            }
            lhs => Condition::Any(vec![lhs, rhs]),
        }
    }
}

impl Not for Condition {
    type Output = Condition;
    fn not(self) -> Condition {
        Condition::Not(Box::new(self))
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
            Self::BuildingUnlocked(building) => write!(f, "unlocked building '{building}'"),
            Self::MilestoneUnlocked(milestone) => write!(f, "unlocked milestone '{milestone}'"),
            Self::Compare(lhs, op, rhs) => write!(f, "{lhs} {op} {rhs}"),
            Self::All(conditions) => fmt_join(f, " and ", conditions),
            Self::Any(conditions) => fmt_join(f, " or ", conditions),
            Self::Not(condition) => write!(f, "not {}", condition),
        }
    }
}

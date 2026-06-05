use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::{Ref, Resolve};
use crate::content::store::Key;
use crate::defs::flag::FlagDef;
use crate::defs::subject::{RawSubject, Subject};
use crate::defs::value::{RawValue, Value};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum CompareOp {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RawCondition {
    Not(Box<RawCondition>),
    And(Box<RawCondition>, Box<RawCondition>),
    Or(Box<RawCondition>, Box<RawCondition>),
    Flag {
        subject: RawSubject,
        flag: Ref<FlagDef>,
    },
    Compare {
        lhs: Box<RawValue>,
        op: CompareOp,
        rhs: Box<RawValue>,
    },
}

#[derive(Debug)]
pub enum Condition {
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Flag {
        subject: Subject,
        flag: Key<FlagDef>,
    },
    Compare {
        lhs: Box<Value>,
        op: CompareOp,
        rhs: Box<Value>,
    },
}

impl Resolve for RawCondition {
    type Output = Condition;

    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        let cond = match self {
            Self::Not(cond) => Condition::Not(cond.resolve(reg)?),
            Self::And(a, b) => Condition::And(a.resolve(reg)?, b.resolve(reg)?),
            Self::Or(a, b) => Condition::Or(a.resolve(reg)?, b.resolve(reg)?),
            Self::Flag { subject, flag } => Condition::Flag {
                subject: subject.resolve(reg)?,
                flag: flag.resolve(reg)?,
            },
            Self::Compare { lhs, op, rhs } => Condition::Compare {
                lhs: lhs.resolve(reg)?,
                op,
                rhs: rhs.resolve(reg)?,
            },
        };
        Ok(cond)
    }
}

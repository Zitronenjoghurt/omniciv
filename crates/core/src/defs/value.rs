use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::{Ref, Resolve};
use crate::content::store::Key;
use crate::defs::condition::{Condition, RawCondition};
use crate::defs::subject::{RawSubject, Subject};
use crate::defs::track::TrackDef;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RawValue {
    Const(f64),
    Product(Box<RawValue>, Box<RawValue>),
    Sum(Box<RawValue>, Box<RawValue>),
    Track {
        subject: RawSubject,
        track: Ref<TrackDef>,
    },
    Condition {
        cond: Box<RawCondition>,
        success: Box<RawValue>,
        failure: Box<RawValue>,
    },
}

impl RawValue {
    pub fn constant(v: impl Into<f64>) -> Self {
        Self::Const(v.into())
    }

    pub fn product(a: impl Into<RawValue>, b: impl Into<RawValue>) -> Self {
        Self::Product(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn sum(a: impl Into<RawValue>, b: impl Into<RawValue>) -> Self {
        Self::Sum(Box::new(a.into()), Box::new(b.into()))
    }
}

#[derive(Debug)]
pub enum Value {
    Const(f64),
    Product(Box<Value>, Box<Value>),
    Sum(Box<Value>, Box<Value>),
    Track {
        subject: Subject,
        track: Key<TrackDef>,
    },
    Condition {
        cond: Box<Condition>,
        success: Box<Value>,
        failure: Box<Value>,
    },
}

impl Resolve for RawValue {
    type Output = Value;

    fn resolve(self, _reg: &Registry) -> ContentResult<Self::Output> {
        let value = match self {
            RawValue::Const(v) => Value::Const(v),
            RawValue::Product(a, b) => Value::Product(a.resolve(_reg)?, b.resolve(_reg)?),
            RawValue::Sum(a, b) => Value::Sum(a.resolve(_reg)?, b.resolve(_reg)?),
            RawValue::Track { subject, track } => Value::Track {
                subject: subject.resolve(_reg)?,
                track: track.resolve(_reg)?,
            },
            RawValue::Condition {
                cond,
                success,
                failure,
            } => Value::Condition {
                cond: cond.resolve(_reg)?,
                success: success.resolve(_reg)?,
                failure: failure.resolve(_reg)?,
            },
        };
        Ok(value)
    }
}

impl From<f64> for RawValue {
    fn from(v: f64) -> Self {
        RawValue::Const(v)
    }
}

impl<T: Into<RawValue>> std::ops::Mul<T> for RawValue {
    type Output = RawValue;
    fn mul(self, rhs: T) -> RawValue {
        RawValue::Product(Box::new(self), Box::new(rhs.into()))
    }
}

impl<T: Into<RawValue>> std::ops::Add<T> for RawValue {
    type Output = RawValue;
    fn add(self, rhs: T) -> RawValue {
        RawValue::Sum(Box::new(self), Box::new(rhs.into()))
    }
}

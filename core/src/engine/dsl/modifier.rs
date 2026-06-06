use crate::engine::dsl::value::Value;
use crate::math::modifiers::{ModifierKind, Modifiers};
use crate::types::stat::Stat;
use crate::State;

#[macro_export]
macro_rules! modifiers {
    ($($stat:expr => $op:tt $v:expr);* $(;)?) => {{
        const M: &[$crate::engine::dsl::modifier::Modifier] =
            &[$( $crate::modifiers!(@m $stat, $op, $v) ),*];
        M
    }};
    (@m $stat:expr, +, $v:expr) => { $crate::engine::dsl::modifier::Modifier { stat: $stat, kind: $crate::math::modifiers::ModifierKind::Add, value: $v } };
    (@m $stat:expr, *, $v:expr) => { $crate::engine::dsl::modifier::Modifier { stat: $stat, kind: $crate::math::modifiers::ModifierKind::Mul, value: $v } };
    (@m $stat:expr, ^, $v:expr) => { $crate::engine::dsl::modifier::Modifier { stat: $stat, kind: $crate::math::modifiers::ModifierKind::Exp, value: $v } };
}

pub struct Modifier {
    pub stat: Stat,
    pub kind: ModifierKind,
    pub value: Value,
}

impl Modifier {
    pub const fn add(stat: Stat, value: Value) -> Self {
        Self {
            stat,
            kind: ModifierKind::Add,
            value,
        }
    }

    pub const fn mul(stat: Stat, value: Value) -> Self {
        Self {
            stat,
            kind: ModifierKind::Mul,
            value,
        }
    }

    pub const fn exp(stat: Stat, value: Value) -> Self {
        Self {
            stat,
            kind: ModifierKind::Exp,
            value,
        }
    }

    pub fn fold_into(&self, state: &State, mods: &mut Modifiers, scale: f64) {
        let value = self.value.resolve(state).as_f64();
        mods.insert(self.kind, value * scale);
    }
}

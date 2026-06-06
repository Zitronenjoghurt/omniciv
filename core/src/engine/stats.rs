use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;
use crate::engine::dsl::{Mutate, Query};
use crate::math::modifiers::Modifiers;
use crate::state::State;
use crate::types::milestone::Milestone;
use crate::types::stat::Stat;
use crate::Building;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub struct StatCtx<'a> {
    pub state: &'a mut State,
}

#[derive(Debug)]
pub struct Stats {
    values: HashMap<Stat, Modifiers>,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

impl Stats {
    pub fn get(&self, stat: Stat) -> f64 {
        self.values.get(&stat).map(|m| m.value()).unwrap_or(0.0)
    }

    pub fn fold_modifier(&mut self, state: &State, modifier: &Modifier, scale: f64) {
        let modifiers = self.values.entry(modifier.stat).or_default();
        modifier.fold_into(state, modifiers, scale);
    }

    pub fn resolve(ctx: &StatCtx<'_>) -> Self {
        let mut acc = Self::default();
        Milestone::iter()
            .filter(|m| ctx.met(Condition::MilestoneUnlocked(*m)))
            .for_each(|m| {
                m.modifiers()
                    .iter()
                    .for_each(|modifier| acc.fold_modifier(ctx.state, modifier, 1.0));
            });
        Building::iter().for_each(|b| {
            let count = ctx.eval(Value::BuildingCount(b)).as_f64();
            if count > 0.0 {
                b.modifiers()
                    .iter()
                    .for_each(|modifier| acc.fold_modifier(ctx.state, modifier, count));
            }
        });
        acc
    }
}

impl Query for StatCtx<'_> {
    fn state(&self) -> &State {
        self.state
    }
}

impl Mutate for StatCtx<'_> {
    fn state_mut(&mut self) -> &mut State {
        self.state
    }
}

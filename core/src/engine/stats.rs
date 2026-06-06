use crate::engine::capabilities::individual_count::IndividualCount;
use crate::engine::capabilities::modifying::Modifying;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::{Mutate, Query};
use crate::math::modifiers::Modifiers;
use crate::state::State;
use crate::types::human::Human;
use crate::types::milestone::Milestone;
use crate::types::stat::Stat;
use crate::Building;
use std::collections::HashMap;

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

    fn fold_modifier(&mut self, state: &State, modifier: &Modifier, scale: f64) {
        let modifiers = self.values.entry(modifier.stat).or_default();
        modifier.fold_into(state, modifiers, scale);
    }

    fn set(&mut self, stat: &Stat, modifiers: Modifiers) {
        self.values.insert(*stat, modifiers);
    }

    pub fn resolve(ctx: &StatCtx<'_>) -> Self {
        let mut acc = Self::default();
        Self::resolve_modifying_type::<Building>(ctx, &mut acc);
        Self::resolve_total_count::<Building>(ctx, &mut acc);
        Self::resolve_modifying_type::<Human>(ctx, &mut acc);
        Self::resolve_total_count::<Human>(ctx, &mut acc);
        Self::resolve_modifying_type::<Milestone>(ctx, &mut acc);
        acc
    }

    fn resolve_modifying_type<M: Modifying>(ctx: &StatCtx, acc: &mut Self) {
        M::iter_all()
            .filter(|m| ctx.met(m.modifying_active()))
            .for_each(|m| {
                let scale = ctx.eval(m.modifying_scale()).as_f64();
                m.modifiers()
                    .iter()
                    .for_each(|modifier| acc.fold_modifier(ctx.state, modifier, scale));
            });
    }

    fn resolve_total_count<T: IndividualCount>(ctx: &StatCtx, acc: &mut Self) {
        let total_count = T::iter_all()
            .map(|t| ctx.eval(t.individual_count()).as_f64())
            .sum::<f64>();
        acc.set(&T::total_count_stat(), Modifiers::from_value(total_count));
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

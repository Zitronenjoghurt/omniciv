use crate::engine::capabilities::auto_unlockable::AutoUnlockable;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::value::Value;
use crate::engine::dsl::{Mutate, Query};
use crate::math::step_resource;
use crate::state::State;
use crate::types::building::Building;
use crate::types::human::Human;
use crate::types::milestone::Milestone;
use crate::types::resource::Resource;
use crate::types::stat::Stat;
use crate::types::technology::Technology;
use strum::IntoEnumIterator;

pub struct TickCtx<'a> {
    state: &'a mut State,
    dt: f64,
}

impl<'a> TickCtx<'a> {
    pub fn new(state: &'a mut State, dt: f64) -> Self {
        Self { state, dt }
    }

    pub fn tick(&mut self) {
        self.check_new_unlocks::<Building>();
        self.check_new_unlocks::<Human>();
        self.check_new_unlocks::<Milestone>();
        self.check_new_unlocks::<Technology>();

        self.state.refresh_stats();

        self.integrate_resources();
    }

    fn check_new_unlocks<U: AutoUnlockable>(&mut self) {
        if self.eval(U::unlock_count()) == U::count().into() {
            return;
        }
        let unlocked: Vec<U> = U::iter_all()
            .filter(|u| !self.met(u.is_unlocked()) && self.met(u.can_unlock()))
            .collect();
        for item in unlocked {
            self.apply(item.unlock_action());
            if let Some(action) = item.on_unlock() {
                self.apply(action);
            };
            if let Some(event) = item.unlock_event() {
                self.apply(Action::TriggerEvent(event));
            }
        }
    }
}

// Mutations
impl<'a> TickCtx<'a> {
    fn integrate_resources(&mut self) {
        if self.dt <= 0.0 {
            return;
        }

        for resource in Resource::iter() {
            let net = self.eval(Value::Stat(Stat::ResourceNet(resource))).as_f64();
            if net == 0.0 {
                continue;
            }
            let current = self.eval(Value::ResourceAmount(resource)).as_f64();
            let next = step_resource(net, current, self.dt, f64::INFINITY);
            self.state.resources.set(resource, next);
        }
    }
}

impl Query for TickCtx<'_> {
    fn state(&self) -> &State {
        self.state
    }
}

impl Mutate for TickCtx<'_> {
    fn state_mut(&mut self) -> &mut State {
        self.state
    }
}

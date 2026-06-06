use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::value::Value;
use crate::engine::dsl::{Mutate, Query};
use crate::engine::event::Event;
use crate::state::State;
use crate::types::building::Building;
use crate::types::milestone::Milestone;
use strum::{EnumCount, IntoEnumIterator};

pub struct TickCtx<'a> {
    state: &'a mut State,
    dt: f64,
}

impl<'a> TickCtx<'a> {
    pub fn new(state: &'a mut State, dt: f64) -> Self {
        Self { state, dt }
    }

    pub fn tick(&mut self) {
        self.milestone_unlocks();
        self.building_unlocks();

        self.state.refresh_stats();
    }

    fn milestone_unlocks(&mut self) {
        if self.eval(Value::MilestonesUnlocked) == Milestone::COUNT.into() {
            return;
        }
        let unlocked: Vec<Milestone> = Milestone::iter()
            .filter(|&m| {
                !self.met(Condition::MilestoneUnlocked(m)) && self.met(m.unlock_condition())
            })
            .collect();
        for m in unlocked {
            self.apply(Action::UnlockMilestone(m));
            if let Some(on_unlock) = m.on_unlock() {
                self.apply(on_unlock);
            }
            self.apply(Action::TriggerEvent(Event::UnlockedMilestone(m)));
        }
    }

    fn building_unlocks(&mut self) {
        if self.eval(Value::BuildingsUnlocked) == Building::COUNT.into() {
            return;
        }
        let unlocked: Vec<Building> = Building::iter()
            .filter(|&b| {
                !self.met(Condition::BuildingUnlocked(b)) && self.met(b.unlock_condition())
            })
            .collect();
        for b in unlocked {
            self.apply(Action::UnlockBuilding(b));
            self.apply(Action::TriggerEvent(Event::UnlockedBuilding(b)));
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

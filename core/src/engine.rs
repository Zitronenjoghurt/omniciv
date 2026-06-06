use crate::engine::command::Command;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::value::Value;
use crate::engine::dsl::{Mutate, Query};
use crate::engine::error::{EngineError, EngineResult};
use crate::state::State;
use crate::types::building::Building;
use crate::Resource;

pub mod capabilities;
pub mod command;
pub mod dsl;
pub mod error;
pub mod event;
pub mod stats;
mod tick;

#[derive(Default)]
pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn tick(&self, state: &mut State, dt: f64) {
        let mut ctx = tick::TickCtx::new(state, dt);
        ctx.tick();
    }

    pub fn dispatch(&self, state: &mut State, command: Command) -> EngineResult<()> {
        let result = match command {
            Command::Build { building, count } => self.handle_build(state, building, count),
            Command::Gather(resource) => self.handle_gather(state, resource),
        };
        state.refresh_stats();
        result
    }
}

impl Engine {
    fn handle_build(&self, state: &mut State, building: Building, count: u128) -> EngineResult<()> {
        if count == 0 {
            return Err(EngineError::InvalidAmount);
        }
        if !state.met(Condition::BuildingUnlocked(building)) {
            return Err(EngineError::Locked);
        }
        if !state.met(building.affordable(count)) {
            return Err(EngineError::Unaffordable);
        }
        for &(resource, amount) in building.build_cost() {
            state.apply(Action::SpendResource(resource, amount * count as f64));
        }
        state.apply(Action::AddBuilding(building, count));
        Ok(())
    }

    fn handle_gather(&self, state: &mut State, resource: Resource) -> EngineResult<()> {
        let amount = state.eval(Value::ResourceGatherAmount(resource)).as_f64();
        if amount > 0.0 {
            state.apply(Action::GainResource(resource, amount));
        };
        Ok(())
    }
}

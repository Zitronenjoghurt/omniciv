use crate::engine::command::Command;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::value::Value;
use crate::engine::dsl::{Mutate, Query};
use crate::engine::error::{EngineError, EngineResult};
use crate::engine::event::Event;
use crate::state::State;
use crate::types::building::Building;
use crate::types::human::Human;
use crate::types::stat::Stat;
use crate::types::technology::Technology;
use crate::Resource;

pub mod capabilities;
pub mod command;
pub mod cost;
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
            Command::AssignHuman { human, count } => self.handle_assign(state, human, count),
            Command::GrowHuman => self.handle_grow(state),
            Command::ResearchTechnology(technology) => self.handle_research(state, technology),
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
        let cost = building.build_cost();
        if !cost.affordable(state, count as f64) {
            return Err(EngineError::Unaffordable);
        }
        for (resource, amount) in cost.resolve(state, count as f64) {
            state.apply(Action::SpendResource(resource, amount));
        }
        state.apply(Action::AddBuilding(building, count));
        Ok(())
    }

    fn handle_gather(&self, state: &mut State, resource: Resource) -> EngineResult<()> {
        let amount = state
            .eval(Value::Stat(Stat::ResourceGather(resource)))
            .as_f64();
        if amount > 0.0 {
            state.apply(Action::GainResource(resource, amount));
        };
        Ok(())
    }

    fn handle_assign(&self, state: &mut State, human: Human, target: u128) -> EngineResult<()> {
        if human == Human::Idle {
            return Err(EngineError::BadSubmit);
        }
        if !state.met(Condition::HumanUnlocked(human)) {
            return Err(EngineError::Locked);
        }
        let current = state.eval(Value::HumanCount(human)).as_f64() as u128;
        if target > current {
            let idle = state.eval(Value::HumanCount(Human::Idle)).as_f64() as u128;
            let moved = (target - current).min(idle);
            if moved > 0 {
                state.apply(Action::MoveHumans {
                    from: Human::Idle,
                    to: human,
                    count: moved,
                });
            }
        } else if target < current {
            state.apply(Action::MoveHumans {
                from: human,
                to: Human::Idle,
                count: current - target,
            });
        }
        Ok(())
    }

    fn handle_grow(&self, state: &mut State) -> EngineResult<()> {
        let cost = Human::grow_cost();
        if !cost.affordable(state, 1.0) {
            return Err(EngineError::Unaffordable);
        }
        for (resource, amount) in cost.resolve(state, 1.0) {
            state.apply(Action::SpendResource(resource, amount));
        }
        state.apply(Action::AddHumans(Human::Idle, 1));
        Ok(())
    }

    fn handle_research(&self, state: &mut State, technology: Technology) -> EngineResult<()> {
        if !state.met(Condition::TechnologyUnlocked(technology)) {
            return Err(EngineError::Locked);
        }
        if state.met(Condition::TechnologyResearched(technology)) {
            return Err(EngineError::BadSubmit);
        }
        let cost = technology.research_cost();
        if !cost.affordable(state, 1.0) {
            return Err(EngineError::Unaffordable);
        }
        for (resource, amount) in cost.resolve(state, 1.0) {
            state.apply(Action::SpendResource(resource, amount));
        }
        state.apply(Action::ResearchTechnology(technology));
        state.apply(Action::TriggerEvent(Event::ResearchedTechnology(
            technology,
        )));
        Ok(())
    }
}

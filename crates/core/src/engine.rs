use crate::content::Content;
use crate::defs::action::Action;
use crate::defs::value::Value;
use crate::engine::command::Command;
use crate::engine::error::EngineResult;
use crate::engine::stats::Stats;
use crate::state::State;
use crate::view::View;

mod command;
mod error;
mod stats;

#[derive(Debug)]
pub struct Engine {
    data: Content,
}

impl Engine {
    pub fn new(data: Content) -> Self {
        Self { data }
    }

    pub fn tick(&self, state: &mut State, dt: f64) {
        let stats = self.resolve_stats(state);
        let mut stats_ctx = stats::StatContext { state, dt };
        stats.apply(&mut stats_ctx);
    }

    pub fn dispatch(&self, state: &mut State, cmd: Command) -> EngineResult<()> {
        Ok(())
    }

    pub fn view(&self, state: &State) -> View {
        View {}
    }
}

// Stat resolution
impl Engine {
    fn resolve_stats(&self, state: &State) -> Stats {
        todo!()
    }
}

// Action handling
impl Engine {
    fn apply_action(&self, state: &mut State, action: Action) {
        match action {
            Action::AddResource { resource, amount } => {
                let amount = self.solve_value(state, &amount);
                state.update_resource(&resource, |v| *v += amount);
            }
        }
    }
}

// Value solving
impl Engine {
    fn solve_value(&self, state: &State, value: &Value) -> f64 {
        match value {
            Value::Const(v) => *v,
            Value::Product(a, b) => self.solve_value(state, a) * self.solve_value(state, b),
            Value::Sum(a, b) => self.solve_value(state, a) + self.solve_value(state, b),
        }
    }
}

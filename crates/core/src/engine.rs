use crate::content::store::Key;
use crate::content::Content;
use crate::defs::action::Action;
use crate::defs::building::BuildingDef;
use crate::defs::condition::{CompareOp, Condition};
use crate::defs::value::Value;
use crate::engine::command::Command;
use crate::engine::error::EngineResult;
use crate::engine::eval::EvalContext;
use crate::engine::stats::Stats;
use crate::state::State;
use crate::view::View;

mod builtins;
mod command;
mod error;
mod eval;
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
        match cmd {
            Command::BuildBuilding(key) => self.handle_build_building(state, key),
        }
    }

    pub fn view(&self, state: &State) -> View {
        View {}
    }
}

// Handle commands
impl Engine {
    fn handle_build_building(&self, state: &mut State, key: Key<BuildingDef>) -> EngineResult<()> {
        todo!()
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
                let amount = self.solve_value(state, &EvalContext::empty(), &amount);
                state.update_resource(&resource, |v| *v += amount);
            }
        }
    }
}

// Value solving
impl Engine {
    fn solve_value(&self, state: &State, ctx: &EvalContext, value: &Value) -> f64 {
        match value {
            Value::Const(v) => *v,
            Value::Product(a, b) => {
                self.solve_value(state, ctx, a) * self.solve_value(state, ctx, b)
            }
            Value::Sum(a, b) => self.solve_value(state, ctx, a) + self.solve_value(state, ctx, b),
            Value::Track { subject, track } => {
                let subject = ctx.resolve_subject(*subject);
                state
                    .get_track(&subject, track)
                    .unwrap_or(self.data.reg.get(track).default)
            }
            Value::Condition {
                cond,
                success,
                failure,
            } => {
                if self.solve_condition(state, ctx, cond) {
                    self.solve_value(state, ctx, success)
                } else {
                    self.solve_value(state, ctx, failure)
                }
            }
        }
    }
}

// Condition solving
impl Engine {
    fn solve_condition(&self, state: &State, ctx: &EvalContext, cond: &Condition) -> bool {
        match cond {
            Condition::Not(cond) => !self.solve_condition(state, ctx, cond),
            Condition::And(a, b) => {
                self.solve_condition(state, ctx, a) && self.solve_condition(state, ctx, b)
            }
            Condition::Or(a, b) => {
                self.solve_condition(state, ctx, a) || self.solve_condition(state, ctx, b)
            }
            Condition::Flag { subject, flag } => {
                let subject = ctx.resolve_subject(*subject);
                state
                    .get_flag(&subject, flag)
                    .unwrap_or(self.data.reg.get(flag).default)
            }
            Condition::Compare { lhs, op, rhs } => {
                let lhs = self.solve_value(state, ctx, lhs);
                let rhs = self.solve_value(state, ctx, rhs);
                match op {
                    CompareOp::Eq => lhs == rhs,
                    CompareOp::Neq => lhs != rhs,
                    CompareOp::Gt => lhs > rhs,
                    CompareOp::Gte => lhs >= rhs,
                    CompareOp::Lt => lhs < rhs,
                    CompareOp::Lte => lhs <= rhs,
                }
            }
        }
    }
}

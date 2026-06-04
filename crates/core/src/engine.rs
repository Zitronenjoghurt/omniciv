use crate::content::Content;
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
        stats.apply(state, dt);
    }

    pub fn dispatch(&self, state: &mut State, cmd: Command) -> EngineResult<()> {
        Ok(())
    }

    pub fn view(&self, state: &State) -> View {
        View {}
    }
}

// Phases
impl Engine {
    fn resolve_stats(&self, state: &State) -> Stats {
        todo!()
    }
}

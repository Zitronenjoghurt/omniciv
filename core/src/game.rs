use crate::engine::command::{Command, Submit};
use crate::engine::error::EngineResult;
use crate::engine::Engine;
use crate::state::State;
use crate::view::View;
use std::time::Instant;

pub struct Game {
    engine: Engine,
    state: State,
    last_tick: Instant,
}

impl Game {
    pub fn new(state: State) -> Self {
        Self {
            engine: Engine::new(),
            state,
            last_tick: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f64();
        self.engine.tick(&mut self.state, dt);
        self.last_tick = now;
    }

    pub fn submit(&mut self, submit: Submit) -> EngineResult<()> {
        self.dispatch(Command::from_submit(submit)?)
    }

    pub fn dispatch(&mut self, command: Command) -> EngineResult<()> {
        self.engine.dispatch(&mut self.state, command)
    }

    pub fn view(&mut self) -> View {
        let view = View::build(&self.state);
        self.state.flush();
        view
    }
}

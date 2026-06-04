use omniciv_core::content::error::ContentResult;
use omniciv_core::engine::Engine;
use omniciv_core::state::State;

#[derive(Debug)]
pub struct Game {
    engine: Engine,
    state: State,
}

impl Game {
    pub fn initialize() -> ContentResult<Self> {
        let content = omniciv_data::build()?;
        let engine = Engine::new(content);
        let state = State::default();
        Ok(Self { engine, state })
    }
}

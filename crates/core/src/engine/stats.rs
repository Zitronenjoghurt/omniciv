use crate::state::State;

#[derive(Debug, Default)]
pub struct Stats {}

impl Stats {
    pub fn apply(&self, state: &mut State, dt: f64) {}
}

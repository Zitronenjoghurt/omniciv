use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::constant::Constant;
use crate::engine::dsl::value::Value;
use crate::state::State;

pub mod action;
pub mod comp;
pub mod condition;
pub mod constant;
pub mod modifier;
pub mod value;

pub trait Query {
    fn state(&self) -> &State;

    fn eval(&self, value: impl Into<Value>) -> Constant {
        value.into().resolve(self.state())
    }

    fn met(&self, condition: Condition) -> bool {
        condition.resolve(self.state())
    }
}

pub trait Mutate: Query {
    fn state_mut(&mut self) -> &mut State;

    fn apply(&mut self, action: Action) {
        action.apply(self.state_mut());
    }
}

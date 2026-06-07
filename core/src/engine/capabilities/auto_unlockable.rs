use crate::engine::capabilities::type_count::TypeCount;
use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::dsl::action::Action;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::value::Value;
use crate::engine::event::Event;

pub trait AutoUnlockable: TypeCount + TypeIter {
    fn unlock_count() -> Value;
    fn is_unlocked(&self) -> Condition;
    fn can_unlock(&self) -> Condition;
    fn unlock_action(&self) -> Action;
    fn unlock_event(&self) -> Option<Event>;
    fn on_unlock(&self) -> Option<Action>;
}

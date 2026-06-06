use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::modifier::Modifier;
use crate::engine::dsl::value::Value;

pub trait Modifying: TypeIter {
    fn modifying_active(&self) -> Condition;
    fn modifying_scale(&self) -> Value;
    fn modifiers(&self) -> &'static [Modifier];
}

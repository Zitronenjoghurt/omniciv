use crate::engine::capabilities::type_iter::TypeIter;
use crate::engine::dsl::value::Value;
use crate::types::stat::Stat;

pub trait IndividualCount: TypeIter {
    fn total_count_stat() -> Stat;
    fn individual_count(&self) -> Value;
}

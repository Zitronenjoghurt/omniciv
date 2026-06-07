use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::types::resource::Resource;

#[derive(Debug, Copy, Clone)]
pub struct Cost<'a> {
    items: &'a [(Resource, Value)],
}

impl Cost<'_> {
    pub const EMPTY: Cost<'static> = Cost::new(&[]);

    pub const fn new(items: &'static [(Resource, Value)]) -> Self {
        Self { items }
    }

    pub fn resolve(&self, q: &impl Query, multiplier: f64) -> Vec<(Resource, f64)> {
        self.items
            .iter()
            .map(|(resource, amount)| (*resource, amount.resolve(q.state()).as_f64() * multiplier))
            .collect()
    }

    pub fn affordable(&self, q: &impl Query, multiplier: f64) -> bool {
        self.resolve(q, multiplier)
            .into_iter()
            .all(|(resource, amount)| q.eval(Value::ResourceAmount(resource)).as_f64() >= amount)
    }

    pub fn max_affordable(&self, q: &impl Query) -> i64 {
        self.items
            .iter()
            .map(|(resource, amount)| {
                let needed = amount.resolve(q.state()).as_f64();
                if needed <= 0.0 {
                    i64::MAX
                } else {
                    (q.eval(Value::ResourceAmount(*resource)).as_f64() / needed).floor() as i64
                }
            })
            .min()
            .unwrap_or(i64::MAX)
    }
}

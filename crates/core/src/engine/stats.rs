use crate::content::store::Key;
use crate::defs::resource::ResourceDef;
use crate::state::State;
use std::collections::HashMap;

mod resources;

pub struct StatContext<'a> {
    pub state: &'a mut State,
    pub dt: f64,
}

#[derive(Debug, Default)]
pub struct Stats {
    pub resources: HashMap<Key<ResourceDef>, resources::ResourceStats>,
}

impl Stats {
    pub fn apply(&self, ctx: &mut StatContext<'_>) {
        for (resource, stats) in self.resources.iter() {
            stats.apply(ctx, resource);
        }
    }
}

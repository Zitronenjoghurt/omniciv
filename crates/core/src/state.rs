use crate::content::store::Key;
use crate::defs::resource::ResourceDef;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct State {
    pub resources: HashMap<Key<ResourceDef>, f64>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_resource(&mut self, resource: &Key<ResourceDef>, f: impl FnOnce(&mut f64)) {
        f(self.resources.entry(*resource).or_default())
    }
}

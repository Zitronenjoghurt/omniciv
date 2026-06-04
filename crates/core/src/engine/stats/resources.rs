use crate::content::store::Key;
use crate::defs::resource::ResourceDef;
use crate::engine::stats::StatContext;
use crate::math::modifiers::Modifiers;
use crate::math::step_resource;

#[derive(Debug, Default)]
pub struct ResourceStats {
    pub capacity: Modifiers,
    pub production: Modifiers,
}

impl ResourceStats {
    pub fn apply(&self, ctx: &mut StatContext, resource: &Key<ResourceDef>) {
        let rate = self.production.value();
        let capacity = self.capacity.value();
        ctx.state
            .update_resource(resource, |v| *v = step_resource(rate, *v, ctx.dt, capacity));
    }
}

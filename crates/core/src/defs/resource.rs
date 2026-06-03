use crate::content::error::ContentResult;
use crate::content::registry::{Registry, Resolvable};
use crate::content::store::Key;
use crate::defs::era::EraDef;

#[derive(bon::Builder)]
pub struct ResourceData {
    #[builder(into)]
    pub era: String,
}

#[derive(Debug)]
pub struct ResourceDef {
    pub era: Key<EraDef>,
}

impl Resolvable for ResourceData {
    type Output = ResourceDef;
    fn resolve(self, _key: Key<Self::Output>, registry: &Registry) -> ContentResult<Self::Output> {
        Ok(ResourceDef {
            era: registry.resolve_id(&self.era)?,
        })
    }
}

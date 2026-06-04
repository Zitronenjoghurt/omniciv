use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::{Ref, Resolve};
use crate::content::store::Key;
use crate::defs::era::EraDef;

#[derive(Debug, bon::Builder, serde::Serialize, serde::Deserialize)]
pub struct ResourceData {
    #[builder(into)]
    pub era: Ref<EraDef>,
}

#[derive(Debug)]
pub struct ResourceDef {
    pub era: Key<EraDef>,
}

impl Resolve for ResourceData {
    type Output = ResourceDef;
    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        Ok(ResourceDef {
            era: self.era.resolve(reg)?,
        })
    }
}

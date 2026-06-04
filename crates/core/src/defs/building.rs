use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::Resolve;
use crate::defs::effect::{Effect, RawEffect};

#[derive(Debug, bon::Builder, serde::Serialize, serde::Deserialize)]
pub struct BuildingData {
    #[builder(default, with = FromIterator::from_iter)]
    pub effects: Vec<RawEffect>,
}

#[derive(Debug)]
pub struct BuildingDef {
    pub effects: Vec<Effect>,
}

impl Resolve for BuildingData {
    type Output = BuildingDef;

    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        Ok(BuildingDef {
            effects: self.effects.resolve(reg)?,
        })
    }
}

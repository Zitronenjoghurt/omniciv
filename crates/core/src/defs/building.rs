use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::Resolve;
use crate::defs::action::{Action, RawAction};
use crate::defs::passive::{Passive, RawPassive};

#[derive(Debug, bon::Builder, serde::Serialize, serde::Deserialize)]
pub struct BuildingData {
    #[builder(default, with = FromIterator::from_iter)]
    pub passives: Vec<RawPassive>,
    #[builder(default, with = FromIterator::from_iter)]
    pub on_build: Vec<RawAction>,
}

#[derive(Debug)]
pub struct BuildingDef {
    pub passives: Vec<Passive>,
    pub on_build: Vec<Action>,
}

impl Resolve for BuildingData {
    type Output = BuildingDef;

    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        Ok(BuildingDef {
            passives: self.passives.resolve(reg)?,
            on_build: self.on_build.resolve(reg)?,
        })
    }
}

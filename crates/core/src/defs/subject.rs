use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::{Ref, Resolve};
use crate::content::store::Key;
use crate::defs::building::BuildingDef;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RawSubject {
    This,
    Building(Ref<BuildingDef>),
}

#[derive(Debug, Copy, Clone)]
pub enum Subject {
    This,
    Building(Key<BuildingDef>),
}

impl Resolve for RawSubject {
    type Output = Subject;

    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        let subject = match self {
            RawSubject::This => Subject::This,
            RawSubject::Building(building) => Subject::Building(building.resolve(reg)?),
        };
        Ok(subject)
    }
}

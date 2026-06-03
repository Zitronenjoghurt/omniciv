use crate::content::error::ContentResult;
use crate::content::registry::{Registry, Resolvable};
use crate::content::store::Key;

#[derive(bon::Builder)]
pub struct EraData {}

#[derive(Debug)]
pub struct EraDef {}

impl Resolvable for EraData {
    type Output = EraDef;

    fn resolve(self, _key: Key<Self::Output>, _registry: &Registry) -> ContentResult<Self::Output> {
        Ok(EraDef {})
    }
}

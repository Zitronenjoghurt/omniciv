use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::Resolve;

#[derive(Debug, bon::Builder, serde::Serialize, serde::Deserialize)]
pub struct EraData {}

#[derive(Debug)]
pub struct EraDef {}

impl Resolve for EraData {
    type Output = EraDef;
    fn resolve(self, _reg: &Registry) -> ContentResult<Self::Output> {
        Ok(EraDef {})
    }
}

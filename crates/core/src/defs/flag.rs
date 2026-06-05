use crate::content::resolve::Resolve;

#[derive(Debug, bon::Builder, serde::Serialize, serde::Deserialize)]
pub struct FlagData {
    default: bool,
}

impl FlagData {
    pub fn new(default: bool) -> Self {
        Self { default }
    }
}

#[derive(Debug)]
pub struct FlagDef {
    pub default: bool,
}

impl Resolve for FlagData {
    type Output = FlagDef;
    fn resolve(
        self,
        _reg: &crate::content::registry::Registry,
    ) -> crate::content::error::ContentResult<Self::Output> {
        Ok(FlagDef {
            default: self.default,
        })
    }
}

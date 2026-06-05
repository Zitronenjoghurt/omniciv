use crate::content::resolve::Resolve;

#[derive(Debug, bon::Builder, serde::Serialize, serde::Deserialize)]
pub struct TrackData {
    default: f64,
}

impl TrackData {
    pub fn new(default: f64) -> Self {
        Self { default }
    }
}

#[derive(Debug)]
pub struct TrackDef {
    pub default: f64,
}

impl Resolve for TrackData {
    type Output = TrackDef;
    fn resolve(
        self,
        _reg: &crate::content::registry::Registry,
    ) -> crate::content::error::ContentResult<Self::Output> {
        Ok(TrackDef {
            default: self.default,
        })
    }
}

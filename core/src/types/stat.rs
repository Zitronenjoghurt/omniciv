use crate::Resource;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Stat {
    ResourceGather(Resource),
    ResourceConsumption(Resource),
    ResourceProduction(Resource),
    TotalBuildings,
    TotalHumans,
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResourceGather(resource) => {
                write!(f, "amount obtained per gathering of '{resource}'")
            }
            Self::ResourceConsumption(resource) => {
                write!(f, "amount consumed of '{resource}'")
            }
            Self::ResourceProduction(resource) => {
                write!(f, "amount produced of '{resource}'")
            }
            Self::TotalBuildings => write!(f, "total number of buildings"),
            Self::TotalHumans => write!(f, "total number of humans"),
        }
    }
}

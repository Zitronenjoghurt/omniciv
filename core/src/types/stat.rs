use crate::Resource;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Stat {
    PowerTurnover,
    HumanProductionEfficiency,
    ResourceGather(Resource),
    ResourceConsumption(Resource),
    ResourceNet(Resource),
    ResourceProduction(Resource),
    TotalBuildings,
    TotalHumans,
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PowerTurnover => {
                write!(f, "total power turnover")
            }
            Self::HumanProductionEfficiency => write!(f, "human production efficiency"),
            Self::ResourceGather(resource) => {
                write!(f, "amount obtained per gathering of '{resource}'")
            }
            Self::ResourceConsumption(resource) => {
                write!(f, "amount consumed of '{resource}'")
            }
            Self::ResourceNet(resource) => {
                write!(f, "net amount gained of '{resource}'")
            }
            Self::ResourceProduction(resource) => {
                write!(f, "amount produced of '{resource}'")
            }
            Self::TotalBuildings => write!(f, "total number of buildings"),
            Self::TotalHumans => write!(f, "total number of humans"),
        }
    }
}

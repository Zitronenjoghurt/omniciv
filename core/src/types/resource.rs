use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumCount, EnumIter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumCount)]
pub enum Resource {
    Berries = 0,
    Ideas = 1,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Berries => write!(f, "Berries"),
            Self::Ideas => write!(f, "Ideas"),
        }
    }
}

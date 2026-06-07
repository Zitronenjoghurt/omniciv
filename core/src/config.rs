use crate::math::quantity::{Energy, Rate};

pub mod resource {
    use super::*;

    pub const BERRY_FOOD_ENERGY: Energy = Energy::from_kilocalories(1.0);
}

pub mod human {
    use super::*;
    pub const STARTING_IDLE_HUMANS: u128 = 3;

    pub const IDLE_BERRY_UPKEEP: Rate = Rate::from_per_second(0.03);

    pub const GATHERER_BERRY_PRODUCTION: Rate = Rate::from_per_second(0.035);
    pub const GATHERER_BERRY_UPKEEP: Rate = Rate::from_per_second(0.03);

    pub const THINKER_IDEA_PRODUCTION: Rate = Rate::from_per_day(2.0);
    pub const THINKER_BERRY_UPKEEP: Rate = Rate::from_per_second(0.03);

    pub const THINKER_UNLOCK_BERRY_NET: Rate = Rate::from_per_second(0.05);

    pub const GROW_BASE_COST: f64 = 10.0;
    pub const GROW_COST_RATIO: f64 = 1.15;
}

pub mod gather {
    pub const BERRIES_PER_GATHER: f64 = 1.0;
}

pub mod technology {
    pub const FIRE_UNLOCK_THINKERS: u128 = 2;
    pub const FIRE_RESEARCH_IDEAS: f64 = 10.0;
}

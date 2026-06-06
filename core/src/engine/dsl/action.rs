use crate::engine::event::Event;
use crate::types::building::Building;
use crate::types::milestone::Milestone;
use crate::types::resource::Resource;
use std::ops::BitOr;

pub enum Action {
    Chain(Vec<Action>),
    TriggerEvent(Event),
    AddBuilding(Building, u128),
    GainResource(Resource, f64),
    SpendResource(Resource, f64),
    UnlockBuilding(Building),
    UnlockMilestone(Milestone),
}

impl Action {
    pub fn apply(self, state: &mut crate::state::State) {
        match self {
            Self::Chain(actions) => actions.into_iter().for_each(|action| action.apply(state)),
            Self::TriggerEvent(event) => state.events.push(event),
            Self::AddBuilding(building, count) => state.buildings.add(building, count),
            Self::GainResource(resource, amount) => state.resources.add(resource, amount),
            Self::SpendResource(resource, amount) => state.resources.sub(resource, amount),
            Self::UnlockBuilding(building) => state.building_unlocks.set(building, true),
            Self::UnlockMilestone(milestone) => state.milestones.set(milestone, true),
        }
    }
}

impl BitOr for Action {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match self {
            Self::Chain(mut v) => {
                v.push(rhs);
                Self::Chain(v)
            }
            lhs => Self::Chain(vec![lhs, rhs]),
        }
    }
}

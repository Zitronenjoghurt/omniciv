use crate::engine::event::Event;
use crate::types::building::Building;
use crate::types::human::Human;
use crate::types::milestone::Milestone;
use crate::types::resource::Resource;
use crate::types::technology::Technology;
use std::ops::BitOr;

pub enum Action {
    Chain(Vec<Action>),
    TriggerEvent(Event),
    AddBuilding(Building, u128),
    AddHumans(Human, u128),
    MoveHumans { from: Human, to: Human, count: u128 },
    GainResource(Resource, f64),
    ResearchTechnology(Technology),
    SpendResource(Resource, f64),
    UnlockBuilding(Building),
    UnlockHuman(Human),
    UnlockMilestone(Milestone),
    UnlockTechnology(Technology),
}

impl Action {
    pub fn apply(self, state: &mut crate::state::State) {
        match self {
            Self::Chain(actions) => actions.into_iter().for_each(|action| action.apply(state)),
            Self::TriggerEvent(event) => state.events.push(event),
            Self::AddBuilding(building, count) => state.buildings.add(building, count),
            Self::AddHumans(human, count) => state.humans.add(human, count),
            Self::MoveHumans { from, to, count } => {
                state.humans.sub(from, count);
                state.humans.add(to, count);
            }
            Self::GainResource(resource, amount) => state.resources.add(resource, amount),
            Self::ResearchTechnology(technology) => state.technologies.set(technology, true),
            Self::SpendResource(resource, amount) => state.resources.sub(resource, amount),
            Self::UnlockBuilding(building) => state.building_unlocks.set(building, true),
            Self::UnlockHuman(human) => state.human_unlocks.set(human, true),
            Self::UnlockMilestone(milestone) => state.milestones.set(milestone, true),
            Self::UnlockTechnology(technology) => state.technology_unlocks.set(technology, true),
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

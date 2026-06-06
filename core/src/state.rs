use crate::engine::event::Event;
use crate::engine::stats::{StatCtx, Stats};
use crate::state::amount_map::AmountMap;
use crate::state::count_map::CountMap;
use crate::state::flag_map::FlagMap;
use crate::types::building::Building;
use crate::types::milestone::Milestone;
use crate::types::resource::Resource;

mod amount_map;
mod count_map;
mod flag_map;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub(crate) buildings: CountMap<Building>,
    pub(crate) building_unlocks: FlagMap<Building>,
    pub(crate) milestones: FlagMap<Milestone>,
    pub(crate) resources: AmountMap<Resource>,
    #[serde(skip, default)]
    pub(crate) events: Vec<Event>,
    #[serde(skip, default)]
    pub(crate) stats: Stats,
}

impl State {
    pub(crate) fn flush(&mut self) {
        self.events.clear();
    }

    pub(crate) fn refresh_stats(&mut self) {
        self.stats = Stats::resolve(&StatCtx { state: self })
    }
}

impl crate::engine::dsl::Query for State {
    fn state(&self) -> &State {
        self
    }
}

impl crate::engine::dsl::Mutate for State {
    fn state_mut(&mut self) -> &mut State {
        self
    }
}

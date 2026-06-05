use crate::content::store::Key;
use crate::defs::building::BuildingDef;
use crate::defs::flag::FlagDef;
use crate::defs::resource::ResourceDef;
use crate::defs::subject::Subject;
use crate::defs::track::TrackDef;
use crate::state::flag_table::FlagTable;
use crate::state::track_table::TrackTable;
use std::collections::HashMap;

mod flag_table;
mod track_table;

#[derive(Debug, Default)]
pub struct State {
    pub building_flags: FlagTable<Key<BuildingDef>>,
    pub building_tracks: TrackTable<Key<BuildingDef>>,
    pub resources: HashMap<Key<ResourceDef>, f64>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_track(&self, subject: &Subject, track: &Key<TrackDef>) -> Option<f64> {
        match subject {
            Subject::Building(key) => self.building_tracks.get(key, track),
            Subject::This => unreachable!("'This' subject is resolved as a track before lookup"),
        }
    }

    pub fn get_flag(&self, subject: &Subject, flag: &Key<FlagDef>) -> Option<bool> {
        match subject {
            Subject::Building(_) => None,
            Subject::This => unreachable!("'This' subject is resolved as a flag before lookup"),
        }
    }

    pub fn update_resource(&mut self, resource: &Key<ResourceDef>, f: impl FnOnce(&mut f64)) {
        f(self.resources.entry(*resource).or_default())
    }
}

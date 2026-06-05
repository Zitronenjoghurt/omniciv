use crate::content::builder::ContentBuilder;
use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::store::Key;
use crate::defs::flag::{FlagData, FlagDef};
use crate::defs::track::{TrackData, TrackDef};

pub fn core_content() -> ContentBuilder {
    let mut c = ContentBuilder::new();
    for t in BuiltinTrack::ALL {
        c.add(t.id(), TrackData::new(t.default()));
    }
    for f in BuiltinFlag::ALL {
        c.add(f.id(), FlagData::new(f.default()));
    }
    c
}

#[derive(Debug)]
pub struct Builtins {
    tracks: [Key<TrackDef>; BuiltinTrack::ALL.len()],
    flags: [Key<FlagDef>; BuiltinFlag::ALL.len()],
}

impl Builtins {
    pub fn resolve(reg: &Registry) -> ContentResult<Self> {
        let mut tracks = [None; BuiltinTrack::ALL.len()];
        for t in BuiltinTrack::ALL {
            tracks[t as usize] = Some(reg.resolve_id::<TrackDef>(t.id())?);
        }
        let mut flags = [None; BuiltinFlag::ALL.len()];
        for f in BuiltinFlag::ALL {
            flags[f as usize] = Some(reg.resolve_id::<FlagDef>(f.id())?);
        }
        Ok(Self {
            tracks: tracks.map(Option::unwrap),
            flags: flags.map(Option::unwrap),
        })
    }
}

impl std::ops::Index<BuiltinTrack> for Builtins {
    type Output = Key<TrackDef>;
    fn index(&self, index: BuiltinTrack) -> &Self::Output {
        &self.tracks[index as usize]
    }
}

impl std::ops::Index<BuiltinFlag> for Builtins {
    type Output = Key<FlagDef>;
    fn index(&self, index: BuiltinFlag) -> &Self::Output {
        &self.flags[index as usize]
    }
}

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum BuiltinTrack {
    Count = 0,
}

impl BuiltinTrack {
    pub const ALL: [BuiltinTrack; 1] = [BuiltinTrack::Count];

    pub fn id(self) -> &'static str {
        match self {
            BuiltinTrack::Count => "count",
        }
    }

    fn default(self) -> f64 {
        match self {
            BuiltinTrack::Count => 0.0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum BuiltinFlag {
    Unlocked = 0,
}

impl BuiltinFlag {
    pub const ALL: [BuiltinFlag; 1] = [BuiltinFlag::Unlocked];

    pub fn id(self) -> &'static str {
        match self {
            BuiltinFlag::Unlocked => "unlocked",
        }
    }

    fn default(self) -> bool {
        match self {
            BuiltinFlag::Unlocked => false,
        }
    }
}

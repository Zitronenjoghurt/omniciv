use crate::content::store::Key;
use crate::defs::track::TrackDef;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct FlagTable<K: Eq + Hash> {
    map: HashMap<K, HashMap<Key<TrackDef>, bool>>,
}

impl<K: Eq + Hash> Default for FlagTable<K> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash> FlagTable<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: K, track: Key<TrackDef>) -> Option<bool> {
        self.map.get(&key).and_then(|map| map.get(&track)).copied()
    }

    pub fn set(&mut self, key: K, track: Key<TrackDef>, value: bool) {
        self.map.entry(key).or_default().insert(track, value);
    }

    pub fn contains(&self, key: K) -> bool {
        self.map.contains_key(&key)
    }

    pub fn keys(&self) -> impl Iterator<Item = K> + '_
    where
        K: Copy,
    {
        self.map.keys().copied()
    }
}

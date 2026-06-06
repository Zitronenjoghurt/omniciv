use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FlagMap<K: Eq + Hash> {
    flags: HashSet<K>,
}

impl<K: Eq + Hash> Default for FlagMap<K> {
    fn default() -> Self {
        Self {
            flags: HashSet::new(),
        }
    }
}

impl<K: Eq + Hash> FlagMap<K> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K: Eq + Hash> FlagMap<K> {
    pub fn set(&mut self, key: K, value: bool) {
        if value {
            self.flags.insert(key);
        } else {
            self.flags.remove(&key);
        }
    }

    pub fn get(&self, key: &K) -> bool {
        self.flags.contains(key)
    }

    pub fn count_set(&self) -> usize {
        self.flags.len()
    }
}

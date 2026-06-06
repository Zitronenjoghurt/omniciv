use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CountMap<K: Eq + Hash> {
    counts: HashMap<K, u128>,
}

impl<K: Eq + Hash> Default for CountMap<K> {
    fn default() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash> CountMap<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &K) -> u128 {
        self.counts.get(key).copied().unwrap_or(0)
    }

    pub fn set(&mut self, key: K, value: impl Into<u128>) {
        let value = value.into();
        if value == 0 {
            self.counts.remove(&key);
        } else {
            self.counts.insert(key, value);
        }
    }

    pub fn add(&mut self, key: K, value: impl Into<u128>) {
        let value = value.into();
        match self.counts.entry(key) {
            Entry::Occupied(mut e) => {
                *e.get_mut() = e.get().saturating_add(value);
            }
            Entry::Vacant(e) => {
                if value != 0 {
                    e.insert(value);
                }
            }
        }
    }

    pub fn sub(&mut self, key: K, value: impl Into<u128>) {
        let value = value.into();
        if let Entry::Occupied(mut e) = self.counts.entry(key) {
            *e.get_mut() = e.get().saturating_sub(value);
            if *e.get() == 0 {
                e.remove();
            }
        }
    }

    pub fn increment(&mut self, key: K) {
        self.add(key, 1u128);
    }
    pub fn decrement(&mut self, key: K) {
        self.sub(key, 1u128);
    }
}

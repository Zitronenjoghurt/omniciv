use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AmountMap<K: Eq + Hash> {
    amounts: HashMap<K, f64>,
}

impl<K: Eq + Hash> Default for AmountMap<K> {
    fn default() -> Self {
        Self {
            amounts: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash> AmountMap<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &K) -> f64 {
        self.amounts.get(key).copied().unwrap_or(0.0)
    }

    pub fn set(&mut self, key: K, value: f64) {
        if value == 0.0 {
            self.amounts.remove(&key);
        } else {
            self.amounts.insert(key, value);
        }
    }

    pub fn add(&mut self, key: K, value: f64) {
        match self.amounts.entry(key) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += value;
                if *e.get() == 0.0 {
                    e.remove();
                }
            }
            Entry::Vacant(e) => {
                if value != 0.0 {
                    e.insert(value);
                }
            }
        }
    }

    pub fn sub(&mut self, key: K, value: f64) {
        if let Entry::Occupied(mut e) = self.amounts.entry(key) {
            *e.get_mut() -= value;
            if *e.get() == 0.0 {
                e.remove();
            }
        }
    }
}

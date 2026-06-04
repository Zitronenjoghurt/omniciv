use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;
use string_interner::DefaultSymbol;

pub struct Key<T>(DefaultSymbol, PhantomData<T>);

impl<T> Key<T> {
    pub fn new(symbol: DefaultSymbol) -> Self {
        Key(symbol, PhantomData)
    }

    pub fn symbol(&self) -> DefaultSymbol {
        self.0
    }

    pub fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

impl<T> std::fmt::Debug for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Key<T> {}

impl<T> Hash for Key<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Key<T> {}

#[derive(Debug)]
pub struct Store<T> {
    declared: HashSet<Key<T>>,
    entries: HashMap<Key<T>, T>,
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self {
            declared: HashSet::new(),
            entries: HashMap::new(),
        }
    }
}

impl<T> Store<T> {
    pub(crate) fn declare(&mut self, key: Key<T>) -> bool {
        if self.entries.contains_key(&key) {
            return false;
        }
        self.declared.insert(key)
    }

    pub(crate) fn is_known(&self, key: Key<T>) -> bool {
        self.declared.contains(&key) || self.entries.contains_key(&key)
    }

    pub(crate) fn insert(&mut self, key: Key<T>, value: T) {
        self.declared.remove(&key);
        self.entries.insert(key, value);
    }

    pub(crate) fn pending_is_empty(&self) -> bool {
        self.declared.is_empty()
    }

    pub fn get(&self, key: Key<T>) -> &T {
        self.entries
            .get(&key)
            .expect("dangling key, bug in build pipeline")
    }

    pub fn contains(&self, key: Key<T>) -> bool {
        self.entries.contains_key(&key)
    }
}

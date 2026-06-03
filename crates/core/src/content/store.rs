use std::collections::HashMap;
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
    entries: HashMap<Key<T>, T>,
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl<T> Store<T> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, key: Key<T>, value: T) {
        self.entries.insert(key, value);
    }

    pub fn get(&self, key: Key<T>) -> &T {
        self.entries.get(&key).expect(
            "dangling key, this is a bug as long as data associated to a key cannot be removed",
        )
    }

    pub fn contains(&self, key: Key<T>) -> bool {
        self.entries.contains_key(&key)
    }
}

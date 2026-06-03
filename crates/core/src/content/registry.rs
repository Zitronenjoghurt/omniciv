use crate::content::error::{ContentError, ContentResult};
use crate::content::store::{Key, Store};
use crate::defs::*;
use string_interner::DefaultStringInterner;

#[derive(Debug, Default)]
pub struct Registry {
    interner: DefaultStringInterner,
    eras: Store<era::EraDef>,
    resources: Store<resource::ResourceDef>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: Registered>(&mut self, id: impl AsRef<str>, value: T) -> Key<T> {
        let key = Key::new(self.interner.get_or_intern(id.as_ref()));
        T::store_mut(self).insert(key, value);
        key
    }

    pub fn add_raw<R: Resolvable>(
        &mut self,
        id: impl AsRef<str>,
        unresolved: R,
    ) -> ContentResult<Key<R::Output>> {
        let key = Key::new(self.interner.get_or_intern(id.as_ref()));
        let resolved = unresolved.resolve(key, self)?;
        self.add(id, resolved);
        Ok(key)
    }

    pub fn get<T: Registered>(&self, key: Key<T>) -> &T {
        T::store(self).get(key)
    }

    pub fn resolve_id<T: Registered>(&self, id: &str) -> ContentResult<Key<T>> {
        let symbol = self
            .interner
            .get(id)
            .ok_or_else(|| ContentError::NotFound {
                type_name: std::any::type_name::<T>(),
                id: id.to_owned(),
            })?;

        let key = Key::new(symbol);
        if T::store(self).contains(key) {
            Ok(key)
        } else {
            Err(ContentError::NotFound {
                type_name: std::any::type_name::<T>(),
                id: id.to_owned(),
            })
        }
    }
}

pub trait Registered: Sized {
    fn store(registry: &Registry) -> &Store<Self>;
    fn store_mut(registry: &mut Registry) -> &mut Store<Self>;
}

macro_rules! impl_registered {
    ($($ty:ty => $field:ident),* $(,)?) => { $(
        impl Registered for $ty {
            fn store(registry: &Registry) -> &Store<Self> { &registry.$field }
            fn store_mut(registry: &mut Registry) -> &mut Store<Self> { &mut registry.$field }
        }
    )* };
}

impl_registered! {
    era::EraDef => eras,
    resource::ResourceDef => resources
}

pub trait Resolvable {
    type Output: Registered;
    fn resolve(self, key: Key<Self::Output>, registry: &Registry) -> ContentResult<Self::Output>;
}

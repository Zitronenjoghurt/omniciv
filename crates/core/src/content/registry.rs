use crate::content::error::{ContentError, ContentResult};
use crate::content::for_each_content_type;
use crate::content::store::{Key, Store};
use string_interner::DefaultStringInterner;

macro_rules! define_registry {
    ($($field:ident : $data:ty => $def:ty),* $(,)?) => {
        #[derive(Debug, Default)]
        pub struct Registry {
            interner: DefaultStringInterner,
            $( $field: Store<$def>, )*
        }

        $(
            impl Registered for $def {
                fn store(registry: &Registry) -> &Store<Self> { &registry.$field }
                fn store_mut(registry: &mut Registry) -> &mut Store<Self> { &mut registry.$field }
            }
        )*
    };
}
for_each_content_type!(define_registry);

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn declare<T: Registered>(&mut self, id: impl AsRef<str>) -> ContentResult<Key<T>> {
        let key = Key::new(self.interner.get_or_intern(id.as_ref()));
        if !T::store_mut(self).declare(key) {
            return Err(ContentError::DuplicateId {
                type_name: std::any::type_name::<T>(),
                id: id.as_ref().to_owned(),
            });
        }
        Ok(key)
    }

    pub fn key<T: Registered>(&self, id: &str) -> Option<Key<T>> {
        self.interner.get(id).map(Key::new)
    }

    pub fn insert<T: Registered>(&mut self, key: Key<T>, value: T) {
        T::store_mut(self).insert(key, value);
    }

    pub fn add<T: Registered>(&mut self, id: impl AsRef<str>, value: T) -> ContentResult<Key<T>> {
        let key = self.declare::<T>(&id)?;
        self.insert(key, value);
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
        if T::store(self).is_known(key) {
            Ok(key)
        } else {
            Err(ContentError::NotFound {
                type_name: std::any::type_name::<T>(),
                id: id.to_owned(),
            })
        }
    }

    #[cfg(debug_assertions)]
    pub fn pending_empty<T: Registered>(&self) -> bool {
        T::store(self).pending_is_empty()
    }
}

pub trait Registered: Sized {
    fn store(registry: &Registry) -> &Store<Self>;
    fn store_mut(registry: &mut Registry) -> &mut Store<Self>;
}

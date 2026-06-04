use crate::content::error::ContentResult;
use crate::content::registry::{Registered, Registry};
use crate::content::store::Key;
use std::marker::PhantomData;

pub trait Resolve {
    type Output;
    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output>;
}

macro_rules! resolve_identity {
    ($($t:ty),* $(,)?) => {$(
        impl Resolve for $t {
            type Output = $t;
            fn resolve(self, _: &Registry) -> ContentResult<Self> { Ok(self) }
        }
    )*};
}
resolve_identity!(
    f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, bool, char, String
);

impl<R: Resolve> Resolve for Box<R> {
    type Output = Box<R::Output>;
    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        Ok(Box::new((*self).resolve(reg)?))
    }
}

impl<R: Resolve> Resolve for Option<R> {
    type Output = Option<R::Output>;
    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        self.map(|r| r.resolve(reg)).transpose()
    }
}

impl<R: Resolve> Resolve for Vec<R> {
    type Output = Vec<R::Output>;
    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        self.into_iter().map(|r| r.resolve(reg)).collect()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ref<T> {
    id: String,
    #[serde(skip)]
    _t: PhantomData<fn() -> T>,
}

impl<T> Ref<T> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            _t: PhantomData,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<T: Registered> Resolve for Ref<T> {
    type Output = Key<T>;
    fn resolve(self, reg: &Registry) -> ContentResult<Key<T>> {
        reg.resolve_id::<T>(&self.id)
    }
}

impl<T> From<&str> for Ref<T> {
    fn from(s: &str) -> Self {
        Ref::new(s)
    }
}

impl<T> From<String> for Ref<T> {
    fn from(s: String) -> Self {
        Ref::new(s)
    }
}

impl<T> From<&String> for Ref<T> {
    fn from(s: &String) -> Self {
        Ref::new(s.clone())
    }
}

impl<T> std::fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ref<{}>({:?})", std::any::type_name::<T>(), self.id)
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Ref::new(self.id.clone())
    }
}

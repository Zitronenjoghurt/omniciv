use crate::content::error::ContentResult;
use crate::content::registry::{Registered, Registry};
use crate::content::resolve::Resolve;
use crate::content::{for_each_content_type, Content};

macro_rules! define_content_builder {
    ($($field:ident : $data:ty => $def:ty),* $(,)?) => {
        #[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct ContentBuilder {
            $( #[serde(default)] $field: Vec<(String, $data)>, )*
        }

        $(
            impl Staged for $data {
                fn bucket_mut(b: &mut ContentBuilder) -> &mut Vec<(String, Self)> {
                    &mut b.$field
                }
            }
        )*

        impl ContentBuilder {
            pub fn build(self) -> ContentResult<Content> {
                let mut reg = Registry::new();
                $( declare_bucket(&self.$field, &mut reg)?; )*
                $( resolve_bucket(self.$field, &mut reg)?; )*
                Ok(Content { reg })
            }

            pub fn merge(&mut self, mut other: ContentBuilder) {
                $( self.$field.append(&mut other.$field); )*
            }
        }
    };
}
for_each_content_type!(define_content_builder);

pub trait Staged: Sized {
    fn bucket_mut(b: &mut ContentBuilder) -> &mut Vec<(String, Self)>;
}

impl ContentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<R: Staged>(&mut self, id: impl AsRef<str>, raw: R) {
        R::bucket_mut(self).push((id.as_ref().to_owned(), raw));
    }
}

fn declare_bucket<R: Resolve>(bucket: &[(String, R)], reg: &mut Registry) -> ContentResult<()>
where
    R::Output: Registered,
{
    for (id, _) in bucket {
        reg.declare::<R::Output>(id)?;
    }
    Ok(())
}

fn resolve_bucket<R: Resolve>(bucket: Vec<(String, R)>, reg: &mut Registry) -> ContentResult<()>
where
    R::Output: Registered,
{
    for (id, raw) in bucket {
        let key = reg
            .key::<R::Output>(&id)
            .expect("every id was declared in pass 1");
        let def = raw.resolve(reg)?;
        reg.insert(key, def);
    }
    debug_assert!(
        reg.pending_empty::<R::Output>(),
        "a declared id of this type was never filled, declare/resolve are out of sync",
    );
    Ok(())
}

#[macro_export]
macro_rules! building {
    ($content:expr, $id:expr, $($chain:tt)*) => {
        $content.add($id, $crate::defs::building::BuildingData::builder()$($chain)*.build())
    };
}

#[macro_export]
macro_rules! era {
    ($content:expr, $id:expr, $($chain:tt)*) => {
        $content.add($id, $crate::defs::era::EraData::builder()$($chain)*.build())
    };
    ($content:expr, $id:expr) => {
        $content.add($id, $crate::defs::era::EraData::builder().build())
    };
}

#[macro_export]
macro_rules! resource {
    ($content:expr, $id:expr, $($chain:tt)*) => {
        $content.add($id, $crate::defs::resource::ResourceData::builder()$($chain)*.build())
    };
}

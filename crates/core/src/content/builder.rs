use crate::content::error::ContentResult;
use crate::content::registry::{Registry, Resolvable};
use crate::content::Content;
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ContentBuilder {
    staged: HashMap<TypeId, Box<dyn Any>>,
}

impl ContentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(mut self) -> ContentResult<Content> {
        let mut reg = Registry::new();

        self.process_type::<crate::defs::era::EraData>(&mut reg)?;
        self.process_type::<crate::defs::resource::ResourceData>(&mut reg)?;

        debug_assert!(
            self.staged.is_empty(),
            "Some registered content was never processed, you probably forgot a process_type call"
        );

        Ok(Content { registry: reg })
    }

    pub fn add<R: Resolvable + 'static>(&mut self, id: impl AsRef<str>, raw: R) {
        let type_id = TypeId::of::<R>();

        let bucket = self
            .staged
            .entry(type_id)
            .or_insert_with(|| Box::new(Vec::<(String, R)>::new()))
            .downcast_mut::<Vec<(String, R)>>()
            .unwrap();

        bucket.push((id.as_ref().to_owned(), raw));
    }

    fn process_type<R: Resolvable + 'static>(&mut self, reg: &mut Registry) -> ContentResult<()> {
        if let Some(boxed_bucket) = self.staged.remove(&TypeId::of::<R>()) {
            let bucket = *boxed_bucket.downcast::<Vec<(String, R)>>().unwrap();

            for (id, raw) in bucket {
                reg.add_raw(id, raw)?;
            }
        }
        Ok(())
    }
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

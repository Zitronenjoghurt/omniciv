pub mod builder;
pub mod error;
pub mod registry;
pub mod resolve;
pub mod store;

#[derive(Debug)]
pub struct Content {
    registry: registry::Registry,
}

macro_rules! for_each_content_type {
    ($m:ident) => {
        $m! {
            buildings: $crate::defs::building::BuildingData => $crate::defs::building::BuildingDef,
            eras:      $crate::defs::era::EraData          => $crate::defs::era::EraDef,
            resources: $crate::defs::resource::ResourceData => $crate::defs::resource::ResourceDef,
        }
    };
}
pub(crate) use for_each_content_type;

pub mod prelude {
    pub use super::builder::ContentBuilder;
    pub use super::error::*;
    pub use super::Content;
    use crate::content::resolve::Ref;
    use crate::defs::passive::RawPassive;
    use crate::defs::resource::ResourceDef;
    use crate::defs::value::RawValue;

    pub fn produce(resource: impl Into<Ref<ResourceDef>>, rate: impl Into<RawValue>) -> RawPassive {
        RawPassive::produce(resource, rate)
    }
}

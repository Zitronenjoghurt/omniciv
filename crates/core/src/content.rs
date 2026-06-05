pub mod builder;
pub mod error;
pub mod registry;
pub mod resolve;
pub mod store;

#[derive(Debug)]
pub struct Content {
    pub reg: registry::Registry,
}

macro_rules! for_each_content_type {
    ($m:ident) => {
        $m! {
            buildings: $crate::defs::building::BuildingData => $crate::defs::building::BuildingDef,
            eras:      $crate::defs::era::EraData          => $crate::defs::era::EraDef,
            flags:     $crate::defs::flag::FlagData        => $crate::defs::flag::FlagDef,
            resources: $crate::defs::resource::ResourceData => $crate::defs::resource::ResourceDef,
            tracks:    $crate::defs::track::TrackData      => $crate::defs::track::TrackDef,
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

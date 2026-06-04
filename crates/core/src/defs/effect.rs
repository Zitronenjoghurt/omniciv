use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::{Ref, Resolve};
use crate::content::store::Key;
use crate::defs::resource::ResourceDef;
use crate::defs::value::{RawValue, Value};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RawEffect {
    Produce {
        resource: Ref<ResourceDef>,
        rate: RawValue,
    },
}

impl RawEffect {
    pub fn produce(resource: impl Into<Ref<ResourceDef>>, rate: impl Into<RawValue>) -> Self {
        Self::Produce {
            resource: resource.into(),
            rate: rate.into(),
        }
    }
}

#[derive(Debug)]
pub enum Effect {
    Produce {
        resource: Key<ResourceDef>,
        rate: Value,
    },
}

impl Resolve for RawEffect {
    type Output = Effect;

    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        let effect = match self {
            Self::Produce { resource, rate } => Effect::Produce {
                resource: resource.resolve(reg)?,
                rate: rate.resolve(reg)?,
            },
        };
        Ok(effect)
    }
}

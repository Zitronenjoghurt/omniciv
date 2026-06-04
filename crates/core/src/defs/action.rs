use crate::content::error::ContentResult;
use crate::content::registry::Registry;
use crate::content::resolve::{Ref, Resolve};
use crate::content::store::Key;
use crate::defs::resource::ResourceDef;
use crate::defs::value::{RawValue, Value};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum RawAction {
    AddResource {
        resource: Ref<ResourceDef>,
        amount: RawValue,
    },
}

impl RawAction {
    pub fn add_resource(
        resource: impl Into<Ref<ResourceDef>>,
        amount: impl Into<RawValue>,
    ) -> Self {
        Self::AddResource {
            resource: resource.into(),
            amount: amount.into(),
        }
    }
}

#[derive(Debug)]
pub enum Action {
    AddResource {
        resource: Key<ResourceDef>,
        amount: Value,
    },
}

impl Resolve for RawAction {
    type Output = Action;

    fn resolve(self, reg: &Registry) -> ContentResult<Self::Output> {
        let action = match self {
            Self::AddResource { resource, amount } => Action::AddResource {
                resource: resource.resolve(reg)?,
                amount: amount.resolve(reg)?,
            },
        };
        Ok(action)
    }
}

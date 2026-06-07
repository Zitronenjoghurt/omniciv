use crate::engine::error::{EngineError, EngineResult};
use crate::types::building::Building;
use crate::types::human::Human;
use crate::types::resource::Resource;
use crate::types::technology::Technology;
use crate::view::form::FormId;

#[derive(Debug, Clone)]
pub enum Command {
    Build { building: Building, count: u128 },
    Gather(Resource),
    AssignHuman { human: Human, count: u128 },
    GrowHuman,
    ResearchTechnology(Technology),
}

impl Command {
    pub(crate) fn from_submit(submit: Submit) -> EngineResult<Self> {
        match (submit.form, submit.values.as_slice()) {
            (FormId::Build(building), [FieldValue::Int(count)]) if *count >= 0 => Ok(Self::Build {
                building,
                count: *count as u128,
            }),
            (FormId::Gather(resource), []) => Ok(Self::Gather(resource)),
            (FormId::Assign(human), [FieldValue::Int(count)]) if *count >= 0 => {
                Ok(Self::AssignHuman {
                    human,
                    count: *count as u128,
                })
            }
            (FormId::GrowHuman, []) => Ok(Self::GrowHuman),
            (FormId::Research(technology), []) => Ok(Self::ResearchTechnology(technology)),
            _ => Err(EngineError::BadSubmit),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Submit {
    pub form: FormId,
    pub values: Vec<FieldValue>,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Choice(usize),
}

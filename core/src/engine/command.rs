use crate::engine::error::{EngineError, EngineResult};
use crate::types::building::Building;
use crate::types::resource::Resource;
use crate::view::form::FormId;

#[derive(Debug, Clone)]
pub enum Command {
    Build { building: Building, count: u128 },
    Gather(Resource),
}

impl Command {
    pub(crate) fn from_submit(submit: Submit) -> EngineResult<Self> {
        match (submit.form, submit.values.as_slice()) {
            (FormId::Build(building), [FieldValue::Int(count)]) if *count >= 0 => Ok(Self::Build {
                building,
                count: *count as u128,
            }),
            (FormId::Gather(resource), []) => Ok(Self::Gather(resource)),
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

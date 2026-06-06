use crate::types::resource::Resource;
use crate::Building;

#[derive(Debug, Clone, Hash)]
pub enum FormId {
    Build(Building),
    Gather(Resource),
}

pub struct Form {
    pub id: FormId,
    pub label: String,
    pub enabled: bool,
    pub fields: Vec<Field>,
    pub notes: Vec<Note>,
}

pub struct Field {
    pub label: String,
    pub input: FieldInput,
}

pub enum FieldInput {
    Toggle {
        value: bool,
    },
    Stepper {
        value: i64,
        min: i64,
        max: i64,
    },
    Slider {
        value: f64,
        min: f64,
        max: f64,
        step: f64,
    },
    Selector {
        selected: usize,
        options: Vec<String>,
    },
}

pub enum Note {
    Cost(Vec<ResourceCost>),
    Tooltip(String),
    Description(String),
}

pub struct ResourceCost {
    pub resource: Resource,
    pub amount: f64,
}

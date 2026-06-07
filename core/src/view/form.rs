use crate::types::human::Human;
use crate::types::resource::Resource;
use crate::types::technology::Technology;
use crate::Building;

#[derive(Debug, Clone, Hash)]
pub enum FormId {
    Build(Building),
    Gather(Resource),
    Assign(Human),
    GrowHuman,
    Research(Technology),
}

pub struct Form {
    pub id: FormId,
    pub label: String,
    pub mode: FormMode,
    pub enabled: bool,
    pub fields: Vec<Field>,
    pub notes: Vec<Note>,
}

pub enum FormMode {
    Action,
    Live,
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
        quick_steps: Vec<i64>,
        allow_max: bool,
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

impl Note {
    pub fn cost(items: impl IntoIterator<Item = (Resource, f64)>) -> Self {
        Note::Cost(
            items
                .into_iter()
                .map(|(resource, amount)| ResourceCost { resource, amount })
                .collect(),
        )
    }
}

pub struct ResourceCost {
    pub resource: Resource,
    pub amount: f64,
}

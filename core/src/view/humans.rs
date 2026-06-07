use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::types::human::Human;
use crate::view::form::{Field, FieldInput, Form, FormId, FormMode, Note};
use crate::view::{Assemble, ViewCtx};

pub struct HumanView {
    pub kind: Human,
    pub count: f64,
    pub forms: Vec<Form>,
}

impl Assemble<Human> for HumanView {
    fn assemble(ctx: &ViewCtx<'_>, kind: Human) -> Self {
        let count = ctx.eval(Value::HumanCount(kind)).as_f64();
        let mut forms = Vec::new();
        if kind == Human::Idle {
            forms.push(grow_form(ctx));
        } else {
            let idle = ctx.eval(Value::HumanCount(Human::Idle)).as_f64();
            forms.push(assign_form(kind, count as u128, idle as u128));
        }
        Self { kind, count, forms }
    }
}

fn grow_form(ctx: &ViewCtx<'_>) -> Form {
    let cost = Human::grow_cost();
    Form {
        id: FormId::GrowHuman,
        label: "Grow human".into(),
        mode: FormMode::Action,
        enabled: cost.affordable(ctx, 1.0),
        fields: vec![],
        notes: vec![Note::cost(cost.resolve(ctx, 1.0))],
    }
}

fn assign_form(kind: Human, current: u128, idle: u128) -> Form {
    Form {
        id: FormId::Assign(kind),
        label: "Assign".into(),
        mode: FormMode::Live,
        enabled: true,
        fields: vec![Field {
            label: "Workers".into(),
            input: FieldInput::Stepper {
                value: current as i64,
                min: 0,
                max: (current + idle) as i64,
                quick_steps: vec![1, 5, 10],
                allow_max: true,
            },
        }],
        notes: vec![],
    }
}

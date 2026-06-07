use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::view::form::{Field, FieldInput, Form, FormId, FormMode, Note};
use crate::view::{Assemble, ViewCtx};
use crate::Building;

pub struct BuildingView {
    pub kind: Building,
    pub count: f64,
    pub forms: Vec<Form>,
}

impl Assemble<Building> for BuildingView {
    fn assemble(ctx: &ViewCtx<'_>, kind: Building) -> Self {
        Self {
            kind,
            count: ctx.eval(Value::BuildingCount(kind)).as_f64(),
            forms: vec![build_form(ctx, kind)],
        }
    }
}

fn build_form(ctx: &ViewCtx<'_>, kind: Building) -> Form {
    let cost = kind.build_cost();
    let max = cost.max_affordable(ctx);
    Form {
        id: FormId::Build(kind),
        label: "Build".into(),
        mode: FormMode::Action,
        enabled: max >= 1,
        fields: vec![Field {
            label: "Amount".into(),
            input: FieldInput::Stepper {
                value: 1,
                min: 1,
                max: max.max(1),
                quick_steps: vec![1, 10],
                allow_max: true,
            },
        }],
        notes: vec![Note::cost(cost.resolve(ctx, 1.0))],
    }
}

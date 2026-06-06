use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::types::resource::Resource;
use crate::view::form::{Field, FieldInput, Form, FormId, Note, ResourceCost};
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
    let max = max_affordable(ctx, cost);
    Form {
        id: FormId::Build(kind),
        label: "Build".into(),
        enabled: max >= 1,
        fields: vec![Field {
            label: "Amount".into(),
            input: FieldInput::Stepper {
                value: 1,
                min: 1,
                max: max.max(1),
            },
        }],
        notes: vec![Note::Cost(
            cost.iter()
                .map(|&(resource, amount)| ResourceCost { resource, amount })
                .collect(),
        )],
    }
}

fn max_affordable(ctx: &ViewCtx<'_>, cost: &[(Resource, f64)]) -> i64 {
    cost.iter()
        .map(|&(resource, amount)| {
            let have = ctx.eval(Value::ResourceAmount(resource)).as_f64();
            if amount <= 0.0 {
                i64::MAX
            } else {
                (have / amount).floor() as i64
            }
        })
        .min()
        .unwrap_or(0)
}

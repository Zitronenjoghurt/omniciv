use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::types::stat::Stat;
use crate::view::form::{Form, FormId, FormMode};
use crate::view::{Assemble, ViewCtx};
use crate::Resource;

pub struct ResourceView {
    pub kind: Resource,
    pub amount: f64,
    pub production: f64,
    pub consumption: f64,
    pub net: f64,
    pub forms: Vec<Form>,
}

impl Assemble<Resource> for ResourceView {
    fn assemble(ctx: &ViewCtx<'_>, kind: Resource) -> Self {
        let mut forms = Vec::new();
        let gather = ctx.eval(Value::Stat(Stat::ResourceGather(kind))).as_f64();
        if gather > 0.0 {
            forms.push(gather_form(kind, gather));
        }
        let production = ctx
            .eval(Value::Stat(Stat::ResourceProduction(kind)))
            .as_f64();
        let consumption = ctx
            .eval(Value::Stat(Stat::ResourceConsumption(kind)))
            .as_f64();
        Self {
            kind,
            amount: ctx.eval(Value::ResourceAmount(kind)).as_f64(),
            production,
            consumption,
            net: production - consumption,
            forms,
        }
    }
}

fn gather_form(kind: Resource, amount: f64) -> Form {
    Form {
        id: FormId::Gather(kind),
        label: format!("Gather +{amount:.2}"),
        mode: FormMode::Action,
        enabled: true,
        fields: vec![],
        notes: vec![],
    }
}

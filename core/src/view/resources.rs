use crate::engine::dsl::value::Value;
use crate::engine::dsl::Query;
use crate::types::stat::Stat;
use crate::view::form::{Form, FormId};
use crate::view::{Assemble, ViewCtx};
use crate::Resource;

pub struct ResourceView {
    pub kind: Resource,
    pub amount: f64,
    pub forms: Vec<Form>,
}

impl Assemble<Resource> for ResourceView {
    fn assemble(ctx: &ViewCtx<'_>, kind: Resource) -> Self {
        let mut forms = Vec::new();
        let amount = ctx.eval(Value::Stat(Stat::ResourceGather(kind))).as_f64();
        if amount > 0.0 {
            forms.push(gather_form(kind, amount));
        }
        Self {
            kind,
            amount: ctx.eval(Value::ResourceAmount(kind)).as_f64(),
            forms,
        }
    }
}

fn gather_form(kind: Resource, amount: f64) -> Form {
    Form {
        id: FormId::Gather(kind),
        label: format!("Gather +{amount:.2}"),
        enabled: true,
        fields: vec![],
        notes: vec![],
    }
}

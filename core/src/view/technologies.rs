use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::Query;
use crate::types::technology::Technology;
use crate::view::form::{Form, FormId, FormMode, Note};
use crate::view::{Assemble, ViewCtx};

pub struct TechnologyView {
    pub kind: Technology,
    pub researched: bool,
    pub forms: Vec<Form>,
}

impl Assemble<Technology> for TechnologyView {
    fn assemble(ctx: &ViewCtx<'_>, kind: Technology) -> Self {
        let researched = ctx.met(Condition::TechnologyResearched(kind));
        let mut forms = Vec::new();
        if !researched {
            forms.push(research_form(ctx, kind));
        }
        Self {
            kind,
            researched,
            forms,
        }
    }
}

fn research_form(ctx: &ViewCtx<'_>, kind: Technology) -> Form {
    let cost = kind.research_cost();
    Form {
        id: FormId::Research(kind),
        label: "Research".into(),
        mode: FormMode::Action,
        enabled: cost.affordable(ctx, 1.0),
        fields: vec![],
        notes: vec![Note::cost(cost.resolve(ctx, 1.0))],
    }
}

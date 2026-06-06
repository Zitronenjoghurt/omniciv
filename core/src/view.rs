use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::Query;
use crate::engine::event::Event;
use crate::state::State;
use crate::types::building::Building;
use crate::types::resource::Resource;
use crate::view::buildings::BuildingView;
use crate::view::resources::ResourceView;
use strum::IntoEnumIterator;

pub mod buildings;
pub mod form;
pub mod resources;

pub struct View {
    pub buildings: Vec<BuildingView>,
    pub events: Vec<Event>,
    pub resources: Vec<ResourceView>,
}

pub(crate) struct ViewCtx<'a> {
    state: &'a State,
}

impl Query for ViewCtx<'_> {
    fn state(&self) -> &State {
        self.state
    }
}

pub(crate) trait Assemble<Input> {
    fn assemble(ctx: &ViewCtx<'_>, input: Input) -> Self;
}

impl View {
    pub(crate) fn build(state: &State) -> Self {
        let ctx = ViewCtx { state };
        Self {
            buildings: Building::iter()
                .filter(|&kind| ctx.met(Condition::BuildingUnlocked(kind)))
                .map(|kind| BuildingView::assemble(&ctx, kind))
                .collect(),
            events: state.events.clone(),
            resources: Resource::iter()
                .map(|kind| ResourceView::assemble(&ctx, kind))
                .collect(),
        }
    }
}

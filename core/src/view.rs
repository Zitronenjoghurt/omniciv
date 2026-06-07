use crate::engine::capabilities::auto_unlockable::AutoUnlockable;
use crate::engine::dsl::condition::Condition;
use crate::engine::dsl::Query;
use crate::engine::event::Event;
use crate::state::State;
use crate::types::building::Building;
use crate::types::human::Human;
use crate::types::resource::Resource;
use crate::types::technology::Technology;
use crate::view::buildings::BuildingView;
use crate::view::humans::HumanView;
use crate::view::resources::ResourceView;
use crate::view::stats::{StatsView, UnlockHint};
use crate::view::technologies::TechnologyView;
use std::fmt::Display;
use strum::IntoEnumIterator;

pub mod buildings;
pub mod form;
pub mod humans;
pub mod resources;
pub mod stats;
pub mod technologies;

pub struct View {
    pub stats: StatsView,
    pub buildings: Vec<BuildingView>,
    pub events: Vec<Event>,
    pub humans: Vec<HumanView>,
    pub resources: Vec<ResourceView>,
    pub technologies: Vec<TechnologyView>,
    pub locked_buildings: Vec<UnlockHint>,
    pub locked_humans: Vec<UnlockHint>,
    pub locked_technologies: Vec<UnlockHint>,
}

pub(crate) struct ViewCtx<'a> {
    state: &'a State,
}

impl Query for ViewCtx<'_> {
    fn state(&self) -> &State {
        self.state
    }
}

fn unlock_hints<T>(ctx: &ViewCtx<'_>) -> Vec<UnlockHint>
where
    T: AutoUnlockable + Display,
{
    T::iter_all()
        .filter(|kind| !ctx.met(kind.is_unlocked()))
        .map(|kind| UnlockHint {
            name: kind.to_string(),
            requirement: kind.can_unlock().to_string(),
        })
        .collect()
}

pub(crate) trait Assemble<Input> {
    fn assemble(ctx: &ViewCtx<'_>, input: Input) -> Self;
}

impl View {
    pub(crate) fn build(state: &State) -> Self {
        let ctx = ViewCtx { state };
        Self {
            stats: StatsView::build(&ctx),
            buildings: Building::iter()
                .filter(|&kind| ctx.met(Condition::BuildingUnlocked(kind)))
                .map(|kind| BuildingView::assemble(&ctx, kind))
                .collect(),
            events: state.events.clone(),
            humans: Human::iter()
                .filter(|&kind| ctx.met(Condition::HumanUnlocked(kind)))
                .map(|kind| HumanView::assemble(&ctx, kind))
                .collect(),
            resources: Resource::iter()
                .map(|kind| ResourceView::assemble(&ctx, kind))
                .collect(),
            technologies: Technology::iter()
                .filter(|&kind| ctx.met(Condition::TechnologyUnlocked(kind)))
                .map(|kind| TechnologyView::assemble(&ctx, kind))
                .collect(),
            locked_buildings: unlock_hints::<Building>(&ctx),
            locked_humans: unlock_hints::<Human>(&ctx),
            locked_technologies: unlock_hints::<Technology>(&ctx),
        }
    }
}

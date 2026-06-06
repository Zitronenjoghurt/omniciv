mod engine;
mod fmt;
mod game;
mod math;
mod state;
mod types;
mod view;

pub use engine::command::{Command, FieldValue, Submit};
pub use engine::error::{EngineError, EngineResult};
pub use game::Game;
pub use state::State;
pub use types::building::Building;
pub use types::resource::Resource;
pub use view::form::{Field, FieldInput, Form, FormId, Note, ResourceCost};
pub use view::{buildings::BuildingView, resources::ResourceView, View};

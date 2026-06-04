use omniciv_core::content::prelude::*;

mod buildings;
mod eras;
mod resources;

pub fn build() -> ContentResult<Content> {
    let mut builder = ContentBuilder::new();
    buildings::build(&mut builder)?;
    eras::build(&mut builder)?;
    resources::build(&mut builder)?;
    builder.build()
}

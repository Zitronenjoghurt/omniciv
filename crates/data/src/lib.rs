use omniciv_core::content::builder::ContentBuilder;
use omniciv_core::content::error::ContentResult;
use omniciv_core::content::Content;

mod eras;
mod resources;

pub fn build() -> ContentResult<Content> {
    let mut builder = ContentBuilder::new();
    eras::build(&mut builder)?;
    resources::build(&mut builder)?;
    builder.build()
}

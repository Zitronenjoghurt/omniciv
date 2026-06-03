use omniciv_core::content::builder::ContentBuilder;
use omniciv_core::content::error::ContentResult;
use omniciv_core::resource;

pub fn build(c: &mut ContentBuilder) -> ContentResult<()> {
    resource!(c, "wood", .era("stone"));
    Ok(())
}

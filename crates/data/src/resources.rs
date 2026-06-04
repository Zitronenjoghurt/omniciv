use omniciv_core::content::prelude::*;
use omniciv_core::resource;

pub fn build(c: &mut ContentBuilder) -> ContentResult<()> {
    resource!(c, "wood", .era("stone"));
    Ok(())
}

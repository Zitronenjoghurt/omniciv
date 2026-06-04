use omniciv_core::content::prelude::*;
use omniciv_core::era;

pub fn build(c: &mut ContentBuilder) -> ContentResult<()> {
    era!(c, "stone");
    Ok(())
}

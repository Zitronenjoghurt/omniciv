use omniciv_core::building;
use omniciv_core::content::prelude::*;

pub fn build(c: &mut ContentBuilder) -> ContentResult<()> {
    building!(c, "hut",
        .effects([
            produce("wood", 1.0)
        ])
    );
    Ok(())
}

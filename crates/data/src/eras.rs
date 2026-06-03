use omniciv_core::content::builder::ContentBuilder;
use omniciv_core::content::error::ContentResult;
use omniciv_core::era;

pub fn build(c: &mut ContentBuilder) -> ContentResult<()> {
    era!(c, "stone");
    Ok(())
}

use std::fmt::Display;

pub fn fmt_join(
    f: &mut std::fmt::Formatter<'_>,
    sep: &str,
    parts: &[impl Display],
) -> std::fmt::Result {
    write!(f, "(")?;
    for (i, c) in parts.iter().enumerate() {
        if i > 0 {
            write!(f, "{sep}")?;
        }
        write!(f, "{c}")?;
    }
    write!(f, ")")
}

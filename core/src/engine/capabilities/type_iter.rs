pub trait TypeIter {
    fn iter_all() -> impl Iterator<Item = Self>;
}

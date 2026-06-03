pub mod builder;
pub mod error;
pub mod registry;
pub mod store;

#[derive(Debug)]
pub struct Content {
    registry: registry::Registry,
}

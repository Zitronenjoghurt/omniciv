pub type EngineResult<T> = Result<T, EngineError>;

#[derive(Debug, thiserror::Error)]
pub enum EngineError {}

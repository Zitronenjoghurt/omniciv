pub type EngineResult<T> = Result<T, EngineError>;

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("submission did not match any known form")]
    BadSubmit,
    #[error("amount must be greater than zero")]
    InvalidAmount,
    #[error("building is not unlocked")]
    Locked,
    #[error("not enough resources")]
    Unaffordable,
}

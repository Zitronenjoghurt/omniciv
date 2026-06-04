pub type ContentResult<T> = Result<T, ContentError>;

#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    #[error("Duplicate id '{id}' for {type_name}")]
    DuplicateId { type_name: &'static str, id: String },
    #[error("Failed to resolve {type_name} '{id}'")]
    NotFound { type_name: &'static str, id: String },
}

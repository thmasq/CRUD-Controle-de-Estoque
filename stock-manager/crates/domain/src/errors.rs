use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Entity already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

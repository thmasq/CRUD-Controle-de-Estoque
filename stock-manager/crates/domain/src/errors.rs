use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub type DomainResult<T> = Result<T, DomainError>;

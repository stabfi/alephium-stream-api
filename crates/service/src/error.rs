use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("Requested resource not found")]
    NotFound,
}

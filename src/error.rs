use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_value::SerializerError),

    #[error("Field not found!")]
    FieldNotFound,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_value::SerializerError),

    #[error("Field not found!")]
    FieldNotFound,

    #[error("Adapter not found!")]
    AdapterNotFound,

    #[error("Provider not found!")]
    ProviderNotFound,

    /// This error should be provided by user when impl to load data
    /// Error should be able to be stringified
    #[error("Something wrong when loading entity: {0}")]
    LoadError(String),
}

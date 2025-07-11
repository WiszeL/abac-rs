use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_value::SerializerError),

    #[error("Adapter not found!")]
    AdapterNotFound,

    #[error("Provider not found!")]
    ProviderNotFound,

    #[error("Subject shouldn't be None!")]
    SubjectNotFound,

    /// This error should be provided by user when impl to load data
    /// Error should be able to be stringified
    #[error("Something wrong when loading entity: {0}")]
    LoadError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

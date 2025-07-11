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
    LoadError(Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    pub fn load_error<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self::LoadError(Box::new(err))
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
/// Error occurring during interaction with storage
pub enum Error {
    /// Error occurred during serialization or deserialization of the entity.
    #[error("error performing serialization or deserialization")]
    Codec,
    /// Error occurred during interaction with database.
    #[error("error occurred in the underlying datastore `{0}`")]
    DatabaseError(Box<dyn std::error::Error + Send + Sync>),
    /// This error should be created with `not_found` macro.
    #[error("resource of type `{0}` was not found at the: {1}")]
    NotFound(&'static str, &'static str),
    // TODO: Do we need this type at all?
    /// Unknown or not expected(by architecture) error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}


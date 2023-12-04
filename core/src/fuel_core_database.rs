use crate::fuel_core_storage_custom::Error as StorageError;

/// The error occurred during work with any of databases.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error occurred during serialization or deserialization of the entity.
    #[error("error performing serialization or deserialization")]
    Codec,
    /// Chain can be initialized once.
    #[error("Failed to initialize chain")]
    ChainAlreadyInitialized,
    /// Chain should be initialized before usage.
    #[error("Chain is not yet initialized")]
    ChainUninitialized,
    /// The version of database or data is invalid (possibly not migrated).
    #[error("Invalid database version, expected {expected:#x}, found {found:#x}")]
    InvalidDatabaseVersion {
        /// the current database version
        found: u32,
        /// the database version expected by this build of fuel-core
        expected: u32,
    },
    /// Not related to database error.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<Error> for StorageError {
    fn from(e: Error) -> Self {
        StorageError::DatabaseError(Box::new(e))
    }
}
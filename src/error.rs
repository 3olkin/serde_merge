use thiserror::Error;

/// Alias for a `Result` with the error type `serde_merge::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// This enum represents all possible errors that can occur.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("given value is not an object")]
    NotObject,
}

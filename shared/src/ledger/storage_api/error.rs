//! Storage API error type, extensible with custom user errors and static string
//! messages.

use thiserror::Error;

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SimpleMessage(&'static str),
    #[error("{0}")]
    Custom(CustomError),
    #[error("{0}: {1}")]
    CustomWithMessage(&'static str, CustomError),
}

/// Result of a storage API call.
pub type Result<T> = std::result::Result<T, Error>;

/// Result extension to easily wrap custom errors into [`enum@Error`].
// This is separate from `ResultExt`, because the implementation requires
// different bounds for `T`.
pub trait ResultExt<T> {
    /// Convert a [`std::result::Result`] into storage_api [`Result`].
    fn into_storage_result(self) -> Result<T>;

    /// Add a static message to a possible error in [`Result`].
    fn wrap_err(self, msg: &'static str) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn into_storage_result(self) -> Result<T> {
        self.map_err(Error::new)
    }

    fn wrap_err(self, msg: &'static str) -> Result<T> {
        self.map_err(|err| Error::wrap(msg, err))
    }
}

impl Error {
    /// Create an [`enum@Error`] from another [`std::error::Error`].
    pub fn new<E>(error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::Custom(CustomError(error.into()))
    }

    /// Create an [`enum@Error`] from a static message.
    #[inline]
    pub const fn new_const(msg: &'static str) -> Self {
        Self::SimpleMessage(msg)
    }

    /// Wrap another [`std::error::Error`] with a static message.
    pub fn wrap<E>(msg: &'static str, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self::CustomWithMessage(msg, CustomError(error.into()))
    }
}

/// A custom error
#[derive(Debug)]
pub struct CustomError(pub Box<dyn std::error::Error + Send + Sync>);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An extension to [`Option`] to allow turning `None` case to an Error from a
/// static string (handy for WASM).
pub trait OptionExt<T> {
    /// Transforms the [`Option<T>`] into a [`Result<T>`], mapping
    /// [`Some(v)`] to [`Ok(v)`] and [`None`] to the given static error
    /// message.
    fn ok_or_err_msg(self, msg: &'static str) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_err_msg(self, msg: &'static str) -> Result<T> {
        self.ok_or_else(|| Error::new_const(msg))
    }
}

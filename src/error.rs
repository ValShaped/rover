//! Contains all-encompassing error type for rover.
//!

use thiserror::Error;

/// A specialized [`Result`] type for overmount errors.
pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    MountError(#[from] libmount::Error),
    #[error("{0}")]
    ArgumentError(String),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(feature = "ron")]
    #[error(transparent)]
    RonError(#[from] ron::Error),
    #[cfg(feature = "ron")]
    #[error(transparent)]
    RonSpannedError(#[from] ron::de::SpannedError),
    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlSerError(#[from] toml::ser::Error),
    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
}

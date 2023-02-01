//! # Overmount
//!
//! Library crate used by Overmount to do filesystem stuff
//!
//! TODO: fix name

use thiserror::Error;

/// A specialized [`Result`] type for Overmount errors.
pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ArgumentError(String),
}

pub mod mount;

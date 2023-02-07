//! GG EZ opaque overlay mounts.
//! For when you don't care about the implementation

pub mod config;
pub mod error;
pub mod overlay;

pub use error::{Error, Result};

#[cfg(test)]
mod tests;


//! # Overmount
//!
//! Library crate used by Overmount to do filesystem stuff
//!
//! TODO: fix name

use thiserror::Error;

pub mod btrfs;

pub mod overlay {
    use std::path::Path;
    #[allow(unused_imports)] // TODO: Mount filesystems
    use sys_mount::{Mount, MountFlags, SupportedFilesystems, Unmount, UnmountFlags};

    #[allow(unused_variables)]
    pub fn mount(src: &Path, dst: &Path) {
        let overlay = Mount::builder().fstype("overlay");
        // TODO: Configure overlay mount args, and mount filesystem
    }
}

/// A specialized [`Result`] type for Overmount errors.
pub type Result<T> = std::result::Result<T, crate::Error>;


#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("Field {1} cannot be assigned to {0}:\n{2}")]
    AssignmentError(String, String, String),
}

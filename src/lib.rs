//! # Overmount
//!
//! Library crate used by rwfus.rs to do filesystem stuff
//!
//! The name is currently aa misnomer...
//! TODO: fix name

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

use thiserror::Error;

pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("Field {1} cannot be assigned to {0}:\n{2}")]
    AssignmentError(String, String, String),
}

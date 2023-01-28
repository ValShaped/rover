//! # btrfs
//! Submodule for interacting with btrfs filesystems
//!
//! Currently only implements formatting btrfs filesystems
#![allow(unused_imports)]

pub mod format;
//pub mod mount;

use std::path::{Path, PathBuf};

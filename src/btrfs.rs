//! # btrfs
//! Submodule for interacting with btrfs filesystems
//!
//! Currently only implements formatting btrfs filesystems
#![allow(unused_imports)]

pub mod format;
use std::path::{Path, PathBuf};
use sys_mount::{Mount, MountBuilder, MountFlags};
pub fn mount(_device: &Path, _destination: &Path) {}

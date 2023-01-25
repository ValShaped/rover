use std::{
    io::Result as IoResult,
    path::Path,
    process::{Command, Output},
};

pub mod format_options;
pub use format_options::*;

pub fn format(file: &Path, label: &str) -> IoResult<Output> {
    //Check path
    match file.try_exists() {
        Ok(_) => {
            let file = file
                .to_str()
                .unwrap_or_else(|| panic!("Path {file:?} exists, but does not yield &str!"));
            Command::new("mkfs.btrfs")
                .args(["-ML", label, file])
                .output()
        }
        Err(e) => Err(e),
    }
}

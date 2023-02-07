//! Mounts an overlay at the specified directory

use crate::Result;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Overlay {
    source: PathBuf,
    upper: PathBuf,
    work: PathBuf,
}

impl Overlay {
    pub fn new(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> Self {
        Overlay::default().source(source).destination(destination)
    }
    pub fn source(mut self, path: impl AsRef<Path>) -> Self {
        self.source = path.as_ref().into();
        self
    }
    pub fn destination(mut self, path: impl AsRef<Path>) -> Self {
        self.upper = path.as_ref().into();
        self.upper.push("upper");
        self.work = path.as_ref().into();
        self.work.push("work");
        self
    }
    pub fn mount(self) -> Result<()> {
        mount(&self.source, &self.upper, &self.work)
    }
}

pub fn mount(
    lower: impl AsRef<Path>,
    upper: impl AsRef<Path>,
    work: impl AsRef<Path>,
) -> Result<()> {
    sudo::escalate_if_needed().expect("This program should be run as root.");
    Ok(libmount::Overlay::writable(
        [lower.as_ref()].into_iter(),
        upper.as_ref(),
        work.as_ref(),
        lower.as_ref(),
    )
    .mount()?)
}

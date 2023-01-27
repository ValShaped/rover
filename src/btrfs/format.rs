//! # Btrfs Formatter
//!
//! A wrapper around mkfs.btrfs
//!
//! Use FormatterOptions to specify the options you want to format with, then
//! format with .build().format();
//!
//! [See options for `mkfs.btrfs`.](https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html#options)
//!
//! ### Example:
//! ```rs
//! // Build a formatter
//! let formatter: Formatter = FormatterOptions::new()
//!     .byte_count(536_870_912_u64)?
//!     .checksum(ChecksumAlgorithm::CRC32C)?
//!     .data(DataProfile::Dup)?
//!     //.features(["zoned"])?
//!     .force()?            // true if called
//!     .label("label")?
//!     .metadata(DataProfile::Dup)?
//!     .mixed()?            // true if called
//!     .no_discard()?       // true if called
//!     .nodesize(16_384_usize)?
//!     .rootdir(PathBuf::from("./testdir"))?
//!     .runtime_features(["quota"])?
//!     .sectorsize(4_096_usize)?
//!     .shrink()?           // true if called; requires rootdir
//!     .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")?
//!     .build();
//! // Format a device
//! formatter.format(path)?;
//! ```

use crate::{Error, Result};
use std::{
    ffi::{OsStr, OsString},
    io::Result as IoResult,
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub const RUNTIME_FEATURES: [&str; 2] = ["quota", "free-space-tree"];

#[derive(Copy, Clone, Debug)]
pub enum DataProfile {
    Raid0,
    Raid1,
    Raid1c3,
    Raid1c4,
    Raid5,
    Raid6,
    Raid10,
    Single,
    Dup,
}
const DATA_PROFILES: [&str; 9] = [
    "raid0", "raid1", "raid1c3", "raid1c4", "raid5", "raid6", "raid10", "single", "dup",
];

#[derive(Clone, Copy, Debug)]
pub enum ChecksumAlgorithm {
    CRC32C,
    XXHash,
    SHA256,
    Blake2,
}
const CHECKSUM_ALGORITHMS: [&str; 4] = ["crc32c", "xxhash", "sha256", "blake2"];

#[derive(Clone, Debug, Default)]
enum FormatOpt {
    #[default]
    None,
    Uint(u64),
    Algo(ChecksumAlgorithm),
    Data(DataProfile),
    List(Vec<String>),
    Path(PathBuf),
    Text(String),
    Bool(bool),
}

/// ### FormatterOptions
/// Representation of [options for `mkfs.btrfs`.](https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html#options)
#[derive(Clone, Debug, Default)]
pub struct FormatterOptions {
    byte_count: FormatOpt,       // Uint
    checksum: FormatOpt,         // Algo
    data: FormatOpt,             // Data
    features: FormatOpt,         // List
    force: FormatOpt,            // Bool
    label: FormatOpt,            // Text
    metadata: FormatOpt,         // Data
    mixed: FormatOpt,            // Bool
    no_discard: FormatOpt,       // Bool
    nodesize: FormatOpt,         // Uint
    rootdir: FormatOpt,          // Path
    runtime_features: FormatOpt, // List
    sectorsize: FormatOpt,       // Uint
    shrink: FormatOpt,           // Bool
    uuid: FormatOpt,             // Text
}

impl FormatterOptions {
    /// Construct a new FormatterOptions
    pub fn new() -> Self {
        FormatterOptions::default()
    }

    /// Specify the size of each device, as seen by the filesystem.
    pub fn byte_count(mut self, byte_count: u64) -> Result<Self> {
        self.byte_count = FormatOpt::Uint(byte_count);
        Ok(self)
    }
    /// Specify the checksum algorithm (as ChecksumAlgorithm.)
    pub fn checksum(mut self, checksum: ChecksumAlgorithm) -> Result<Self> {
        self.checksum = FormatOpt::Algo(checksum);
        Ok(self)
    }
    /// Specify the profile for data block groups (as DataProfile.)
    pub fn data(mut self, data: DataProfile) -> Result<Self> {
        self.data = FormatOpt::Data(data);
        Ok(self)
    }
    /// Set mkfs-time features. Unset features by prefixing them with '^'.
    pub fn features<'a>(mut self, features: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        self.features = FormatOpt::List(
            features
                .into_iter()
                .map(|x| -> String { x.to_owned() })
                .collect(),
        );
        Ok(self)
    }
    /// Force-format the device, even if an existing filesystem is present.
    pub fn force(mut self) -> Result<Self> {
        self.force = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Set the partition label.
    pub fn label(mut self, label: &str) -> Result<Self> {
        self.label = FormatOpt::Text(String::from(label));
        Ok(self)
    }
    /// Specify the profile for metadata block groups (as DataProfile.)
    pub fn metadata(mut self, metadata: DataProfile) -> Result<Self> {
        self.metadata = FormatOpt::Data(metadata);
        Ok(self)
    }
    /// Enable mixing of data and metadata blocks
    pub fn mixed(mut self) -> Result<Self> {
        self.mixed = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Disable implicit TRIM of storage device.
    pub fn no_discard(mut self) -> Result<Self> {
        self.no_discard = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Specify the size of a b-tree node
    ///
    /// `nodesize must be a power of 2 less than 2^14
    pub fn nodesize(mut self, nodesize: usize) -> Result<Self> {
        if nodesize.is_power_of_two() && nodesize <= 16384 {
            self.nodesize = FormatOpt::Uint(nodesize as u64);
            Ok(self)
        } else {
            Err(Error::AssignmentError(
                String::from("node_size"),
                format!("{nodesize}"),
                String::from("Must be a power of 2, and <= 16384"),
            ))
        }
    }
    /// Specify a directory containing data to copy into the btrfs filesystem.
    pub fn rootdir(mut self, rootdir: PathBuf) -> Result<Self> {
        self.rootdir = FormatOpt::Path(rootdir);
        Ok(self)
    }
    /// Set runtime features. Unset features by prefixing them with '^'.
    pub fn runtime_features<'a>(
        mut self,
        features: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self> {
        self.runtime_features = FormatOpt::List(
            features
                .into_iter()
                .map(|x| -> String { x.to_owned() })
                .collect(),
        );
        Ok(self)
    }
    /// Set sector size.
    ///
    /// WARNING: If set improperly, the resulting volume will not be mountable.
    pub fn sectorsize(mut self, sectorsize: usize) -> Result<Self> {
        self.sectorsize = FormatOpt::Uint(sectorsize as u64);
        Ok(self)
    }
    /// If the specified device is a file, shrink the file to the minimum required size
    pub fn shrink(mut self) -> Result<Self> {
        self.shrink = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Set the partition UUID
    pub fn uuid(mut self, uuid: &str) -> Result<Self> {
        self.uuid = FormatOpt::Text(uuid.to_owned());
        Ok(self)
    }

    fn to_args(&self) -> Vec<OsString> {
        let mut args = vec![];
        if let FormatOpt::Uint(arg) = self.byte_count {
            args.push(OsString::from(format!("--byte-count={arg}")));
        }
        if let FormatOpt::Algo(arg) = self.checksum {
            args.push(OsString::from(format!(
                "--checksum={}",
                CHECKSUM_ALGORITHMS[arg as usize]
            )));
        }
        if let FormatOpt::Data(arg) = self.data {
            args.push(OsString::from(format!(
                "--data={}",
                DATA_PROFILES[arg as usize]
            )));
        }
        if let FormatOpt::List(features) = &self.features {
            args.push(OsString::from(format!("--features={}", features.join(","))));
        }
        if let FormatOpt::Bool(true) = self.force {
            args.push(OsString::from("--force"))
        }
        if let FormatOpt::Text(arg) = &self.label {
            args.push(OsString::from(format!("--label={arg}")));
        }
        if let FormatOpt::Data(arg) = self.metadata {
            args.push(OsString::from(format!(
                "--metadata={}",
                DATA_PROFILES[arg as usize]
            )));
        }
        if let FormatOpt::Bool(true) = self.mixed {
            args.push(OsString::from("--mixed".to_string()));
        }
        if let FormatOpt::Bool(true) = self.no_discard {
            args.push(OsString::from("--nodiscard".to_string()));
        }
        if let FormatOpt::Uint(arg) = self.nodesize {
            args.push(OsString::from(format!("--nodesize={arg}")));
        }
        if let FormatOpt::List(features) = &self.runtime_features {
            args.push(OsString::from(format!(
                "--runtime-features={}",
                features.join(",")
            )));
        }
        if let FormatOpt::Path(arg) = &self.rootdir {
            args.push(OsString::from(format!("--rootdir={}", arg.display())));
        }
        if let FormatOpt::Bool(true) = self.shrink {
            args.push(OsString::from("--shrink".to_string()));
        }
        args
    }
    pub fn dump_args(self) {
        println!(
            "{:#?}",
            self.to_args().join(OsString::from(" ").as_os_str())
        )
    }

    pub fn build(self) -> Formatter {
        let args = self.to_args();
        Formatter { args }
    }
}

#[doc = r"
### Formatter

A rusty-ish(?) wrapper for mkfs.btrfs. I tried!
"]
#[derive(Debug)]
pub struct Formatter {
    args: Vec<OsString>,
}

impl Formatter {
    /// Specify FormatterOptions first, then build a formatter
    pub fn builder() -> FormatterOptions {
        FormatterOptions::default()
    }
    /// Format a device with mkfs.btrfs
    pub fn format(mut self, device: &Path) -> IoResult<Output> {
        device.try_exists()?;
        self.args.push(OsString::from(device));
        Command::new("mkfs.btrfs").args(self.args).output()
    }
}

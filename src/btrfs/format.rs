//! # Btrfs Formatter
//! [Requires btrfs-progs.](https://btrfs.readthedocs.io/en/latest/Introduction.html)
//!
//! A wrapper around `mkfs.btrfs`
//!
//! Use `FormatterOptions` to specify the options you want to format with, then
//! format with `.finalize().format();`
//!
//! Documentation for [`mkfs.btrfs`: here](https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html).
//!
//! # Example:
//! ```
//! use std::path::PathBuf;
//! use overmount::btrfs::format::{
//!     ChecksumAlgorithm::CRC32C,
//!     DataProfile,
//!     Formatter,
//! };
//! // Configure a formatter
//! let formatter = Formatter::options()
//!     // These are all optional
//!     .byte_count(536_870_912_u64).unwrap()
//!     .checksum(CRC32C).unwrap()
//!     .data(DataProfile::Dup).unwrap()
//!     .features(["mixed-bg"]).unwrap()
//!     .force().unwrap()      // true if called
//!     .label("label").unwrap()
//!     .metadata(DataProfile::Dup).unwrap()
//!     .mixed().unwrap()      // true if called
//!     .no_discard().unwrap() // true if called
//!     .nodesize(4096_usize).unwrap()
//!     .rootdir(PathBuf::from("./testdir")).unwrap()
//!     .runtime_features(["quota"]).unwrap()
//!     .sectorsize(4096_usize).unwrap()
//!     .shrink().unwrap()     // true if called
//!     .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba").unwrap()
//!     // build the Formatter
//!     .finalize();
//! // Format a device
//! formatter.format(&PathBuf::from("./test.btrfs")).unwrap();
//! ```

use crate::{Error, Result};
use std::{
    ffi::{OsStr, OsString},
    fmt::{write, Display},
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

impl std::fmt::Display for DataProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DataProfile::*;
        let data_profile: &str = match *self {
            Raid0 => "raid0",
            Raid1 => "raid1",
            Raid1c3 => "raid1c3",
            Raid1c4 => "raid1c4",
            Raid5 => "raid5",
            Raid6 => "raid6",
            Raid10 => "raid10",
            Single => "single",
            Dup => "dup",
        };
        write!(f, "{data_profile}")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ChecksumAlgorithm {
    CRC32C,
    XXHash,
    SHA256,
    Blake2,
}
impl std::fmt::Display for ChecksumAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ChecksumAlgorithm::*;
        let algorithm: &str = match *self {
            CRC32C => "crc32c",
            XXHash => "xxhash",
            SHA256 => "sha256",
            Blake2 => "blake2",
        };
        write!(f, "{algorithm}")
    }
}

/// It's like an Option, but THICC
#[derive(Clone, Debug, Default)]
enum FormatOpt {
    #[default]
    None,
    Algo(ChecksumAlgorithm),
    Bool(bool),
    Data(DataProfile),
    List(Vec<String>),
    Path(PathBuf),
    Text(String),
    Uint(u64),
    Uuid(String),
}

impl std::fmt::Display for FormatOpt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatOpt::None => write!(f, "None"),
            FormatOpt::Algo(arg) => write!(f, "{arg}"),
            FormatOpt::Bool(arg) => write!(f, "{arg}"),
            FormatOpt::Data(arg) => write!(f, "{arg}"),
            FormatOpt::List(arg) => write!(f, "{}", arg.join(",")),
            FormatOpt::Path(arg) => write!(f, "{}", arg.display()),
            FormatOpt::Text(arg) => write!(f, "{arg}"),
            FormatOpt::Uint(arg) => write!(f, "{arg}"),
            FormatOpt::Uuid(arg) => write!(f, "{arg}"),
        }
    }
}

/// ### FormatterOptions
/// Representation of options for [`mkfs.btrfs`](https://btrfs.readthedocs.io/en/latest/mkfs.btrfs.html#options).
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
    uuid: FormatOpt,             // Uuid
}

impl FormatterOptions {
    /// Specify the size of each device, as seen by the filesystem.
    /// 
    /// # Example
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .byte_count(536_870_912_u64)
    ///     .unwrap();
    /// ```
    pub fn byte_count(mut self, byte_count: u64) -> Result<Self> {
        self.byte_count = FormatOpt::Uint(byte_count);
        Ok(self)
    }
    /// Specify the checksum algorithm (as ChecksumAlgorithm.)
    /// 
    /// # Example
    /// ```
    /// use overmount::btrfs::format::{
    /// *,
    /// ChecksumAlgorithm::CRC32C
    /// };
    /// Formatter::options()
    ///     .checksum(CRC32C)
    ///     .unwrap();
    /// ```
    pub fn checksum(mut self, checksum: ChecksumAlgorithm) -> Result<Self> {
        self.checksum = FormatOpt::Algo(checksum);
        Ok(self)
    }
    /// Specify the profile for data block groups (as DataProfile.)
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::{DataProfile, Formatter};
    /// Formatter::options()
    ///     .data(DataProfile::Dup)
    ///     .unwrap();
    /// ```
    pub fn data(mut self, data: DataProfile) -> Result<Self> {
        self.data = FormatOpt::Data(data);
        Ok(self)
    }
    /// Set mkfs-time features. Unset features by prefixing them with '^'.
    ///
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .features(["mixed-bg"])
    ///     .unwrap();
    /// ```
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
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .force()
    ///     .unwrap();
    /// ```
    pub fn force(mut self) -> Result<Self> {
        self.force = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Set the partition label.
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .label("ExampleLabel")
    ///     .unwrap();
    /// ```
    pub fn label(mut self, label: &str) -> Result<Self> {
        self.label = FormatOpt::Text(String::from(label));
        Ok(self)
    }
    /// Specify the profile for metadata block groups (as DataProfile.)
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::{DataProfile, Formatter};
    /// Formatter::options()
    ///     .metadata(DataProfile::Dup)
    ///     .unwrap();
    /// ```
    pub fn metadata(mut self, metadata: DataProfile) -> Result<Self> {
        self.metadata = FormatOpt::Data(metadata);
        Ok(self)
    }
    /// Enable mixing of data and metadata blocks
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .mixed()
    ///     .unwrap();
    /// ```
    pub fn mixed(mut self) -> Result<Self> {
        self.mixed = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Disable implicit TRIM of storage device.
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .no_discard()
    ///     .unwrap();
    /// ```
    pub fn no_discard(mut self) -> Result<Self> {
        self.no_discard = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Specify the size of a b-tree node
    ///
    /// `nodesize must be a power of 2 less than 2^14
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .label("ExampleLabel")
    ///     .unwrap();
    /// ```
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
    /// 
    /// # Example:
    /// ```
    /// use std::path::PathBuf;
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .rootdir(PathBuf::from("./testdir"))
    ///     .unwrap();
    /// ```
    pub fn rootdir(mut self, rootdir: PathBuf) -> Result<Self> {
        self.rootdir = FormatOpt::Path(rootdir);
        Ok(self)
    }
    /// Set runtime features.
    /// Unset features by prefixing them with '^'.
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .runtime_features(["quota"])
    ///     .unwrap();
    /// ```
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
    /// *If set to a value unsupported by the current kernel,*
    /// *the resulting volume will not be mountable.*
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .sectorsize(4096_usize)
    ///     .unwrap();
    /// ```
    pub fn sectorsize(mut self, sectorsize: usize) -> Result<Self> {
        self.sectorsize = FormatOpt::Uint(sectorsize as u64);
        Ok(self)
    }
    /// If the specified device is a file, and the `rootdir` option is specified,
    /// shrink the file to the minimum required size
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .shrink()
    ///     .unwrap();
    /// ```
    pub fn shrink(mut self) -> Result<Self> {
        self.shrink = FormatOpt::Bool(true);
        Ok(self)
    }
    /// Set the partition UUID
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .uuid("73e1b7e2-a3a8-49c2-b258-06f01a889bba")
    ///     .unwrap();
    /// ```
    pub fn uuid(mut self, uuid: &str) -> Result<Self> {
        self.uuid = FormatOpt::Uuid(uuid.to_owned());
        Ok(self)
    }

    /// Convert self into args (AKA Vec<OsString>)
    fn to_args(&self) -> Vec<OsString> {
        use FormatOpt::*;

        let mut args = vec![];
        let mut push_arg = |arg, opt: &FormatOpt| {
            args.push(OsString::from(match *opt {
                None => return,
                Bool(_) => format!("--{arg}"),
                _ => format!("--{arg}={opt}"),
            }));
        };
        if let Uint(_) = self.byte_count {
            push_arg("byte-count", &self.byte_count);
        }
        if let Algo(_) = self.checksum {
            push_arg("checksum", &self.checksum);
        }
        if let Data(_) = self.data {
            push_arg("data", &self.data);
        }
        if let List(_) = self.features {
            push_arg("features", &self.features);
        }
        if let Bool(_) = self.force {
            push_arg("force", &self.force);
        }
        if let Text(_) = &self.label {
            push_arg("label", &self.label);
        }
        if let Data(_) = self.metadata {
            push_arg("metadata", &self.metadata);
        }
        if let Bool(_) = self.mixed {
            push_arg("mixed", &self.mixed);
        }
        if let Bool(_) = self.no_discard {
            push_arg("nodiscard", &self.no_discard);
        }
        if let Uint(_) = self.nodesize {
            push_arg("nodesize", &self.nodesize);
        }
        if let Path(_) = &self.rootdir {
            push_arg("rootdir", &self.rootdir);
        }
        if let List(_) = &self.runtime_features {
            push_arg("runtime-features", &self.runtime_features);
        }
        if let Uint(_) = self.sectorsize {
            push_arg("sectorsize", &self.sectorsize);
        }
        if let Bool(_) = self.shrink {
            push_arg("shrink", &self.shrink);
        }
        if let Uuid(_) = &self.uuid {
            push_arg("uuid", &self.uuid);
        }
        args
    }

    /// Dump FormatterOptions as they'll be passed to mkfs.btrfs
    /// 
    /// # Example:
    /// ```
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .dump_args();
    /// ```
    pub fn dump_args(self) -> Self {
        println!("{:#?}", self.to_args());
        self
    }

    /// Bake FormatterOptions into a Formatter
    /// 
    /// # Example:
    /// ```
    /// use std::path::PathBuf;
    /// use overmount::btrfs::format::Formatter;
    /// Formatter::options()
    ///     .label("my-Btrfs-volume").unwrap()
    ///     .rootdir(PathBuf::from("./testdir")).unwrap()
    ///     .shrink().unwrap()
    ///     .finalize();
    /// ```
    pub fn finalize(&self) -> Formatter {
        let args = self.to_args();
        Formatter { args }
    }
}

/// ### Formatter
/// A rusty-ish(?) wrapper for mkfs.btrfs. I tried!
#[derive(Debug)]
pub struct Formatter {
    args: Vec<OsString>,
}

impl Formatter {
    /// Specify FormatterOptions first, then build a formatter
    pub fn options() -> FormatterOptions {
        FormatterOptions::default()
    }
    /// Format a device with mkfs.btrfs
    /// 
    /// # Example:
    /// ```
    /// use std::path::PathBuf;
    /// use overmount::btrfs::format::*;
    /// Formatter::options()
    ///     .label("my-Btrfs-volume").unwrap()
    ///     .rootdir(PathBuf::from("./testdir")).unwrap()
    ///     .shrink().unwrap()
    ///     .finalize()
    ///     .format(&PathBuf::from("./test.btrfs")).unwrap();
    /// ```
    pub fn format(mut self, device: &Path) -> IoResult<Output> {
        device.try_exists()?;
        self.args.push(OsString::from(device));
        Command::new("mkfs.btrfs").args(self.args).output()
    }
}

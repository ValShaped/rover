//! Format Options
//!
//! Type [FormatOptions] represents the command-line arguments accepted by mkfs.btrfs
//!

use std::path::PathBuf;

pub const RUNTIME_FEATURES: [&str; 2] = ["quota", "free-space-tree"];

pub const DATA_PROFILES: [&str; 9] = [
    "raid0", "raid1", "raid1c3", "raid1c4", "raid5", "raid6", "raid10", "single", "dup",
];
pub enum DataProfiles {
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

pub struct FormatOptions {
    byte_count: Option<u64>,
    checksum: Option<String>,
    data: Option<DataProfiles>,
    metadata: Option<DataProfiles>,
    mixed: Option<bool>,
    node_size: Option<u32>,
    label: String,
    no_discard: Option<bool>,
    rootdir: Option<PathBuf>,
    shrink: Option<bool>,
    features: Option<Vec<String>>,
    runtime_features: Option<Vec<String>>,
    force: Option<bool>,
    uuid: Option<String>,
}

impl FormatOptions {
    /// byte_count
    pub fn byte_count(mut self, byte_count: u64) -> Self {
        self.byte_count = Some(byte_count);
        self
    }
    pub fn checksum(mut self, checksum: &str) -> Self {
        self.checksum = Some(checksum.to_string());
        self
    }
    pub fn data(mut self, data: DataProfiles) -> Self {
        self.data = Some(data);
        self
    }
    pub fn metadata(mut self, metadata: DataProfiles) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn mixed(mut self, mixed: bool) -> Self {
        self.mixed = Some(mixed);
        self
    }

    pub fn node_size(mut self, node_size: u32) -> Result<Self, String> {
        if node_size.is_power_of_two() && node_size <= 16384 {
            self.node_size = Some(node_size);
            Ok(self)
        } else {
            Err(format!("{node_size} must be a power of 2, and <= 16384"))
        }
    }
    pub fn label(mut self, label: String) -> Result<Self, String> {
        if !label.is_empty() && label.len() < 256 {
            self.label = label;
            Ok(self)
        } else {
            Err(format!("{:?}", label))
        }
    }

    pub fn no_discard(mut self, no_discard: bool) -> Self {
        self.no_discard = Some(no_discard);
        self
    }
    pub fn rootdir(mut self, rootdir: PathBuf) -> Self {
        self.rootdir = Some(rootdir);
        self
    }
    pub fn shrink(mut self, shrink: bool) -> Self {
        self.shrink = Some(shrink);
        self
    }
    pub fn features(mut self, features: Vec<String>) -> Self {
        self.features = Some(features);
        self
    }
    pub fn runtime_features(mut self, runtime_features: Vec<String>) -> Self {
        self.runtime_features = Some(runtime_features);
        self
    }
    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }
    pub fn uuid(mut self, uuid: String) -> Self {
        self.uuid = Some(uuid);
        self
    }
}

impl Default for FormatOptions {
    fn default() -> Self {
        FormatOptions {
            byte_count: None,
            checksum: None,
            data: None,
            metadata: None,
            mixed: None,
            node_size: None,
            label: "btrfs_volume".to_owned(),
            no_discard: None,
            rootdir: None,
            shrink: None,
            features: None,
            runtime_features: None,
            force: None,
            uuid: None,
        }
    }
}

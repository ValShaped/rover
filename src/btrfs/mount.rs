//! Mounter
//!
//! Mount a file with a btrfs filesystem in it as a loopback device

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

//use sys_mount::{Mount, MountBuilder, MountFlags};
//pub fn mount(_device: &Path, _destination: &Path) {}

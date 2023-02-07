use std::path::{Path, PathBuf};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Converts any `IntoIterator<I>` into `Vec<U>`.
fn cast_all<I, T, U>(iter: T) -> Vec<U>
where
    I: Into<U>,
    T: IntoIterator<Item = I>,
{
    iter.into_iter().map(|x: I| -> U { x.into() }).collect()
}

/// # [Common]
/// All other paths, if left unspecified, derive from this one
///
/// | Name             | Default                        |
/// |------------------|--------------------------------|
/// | Base_Directory   | /opt/rwfus                     |
/// | Directories      | /usr /etc/pacman.d /var/lib/pacman /var/cache/pacman
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Common {
    base_directory: PathBuf,
    directories: Vec<PathBuf>,
}

impl Default for Common {
    fn default() -> Self {
        let path = Path::new("");
        Common {
            base_directory: path.join("/opt/rwfus"),
            directories: vec![
                path.join("/usr"),
                path.join("/etc/pacman.d"),
                path.join("/var/lib/pacman"),
                path.join("/var/cache/pacman"),
            ],
        }
    }
}

impl<T: AsRef<Path>> From<T> for Common {
    fn from(value: T) -> Self {
        Common::default().base_directory(value)
    }
}

impl Common {
    pub fn base_directory(mut self, path: impl AsRef<Path>) -> Self {
        self.base_directory = path.as_ref().to_path_buf();
        self
    }
    pub fn directories<I, T>(mut self, directories: T) -> Self
    where
        I: AsRef<Path>,
        T: IntoIterator<Item = I>,
    {
        self.directories = directories
            .into_iter()
            .map(|x: I| -> PathBuf { x.as_ref().to_path_buf() })
            .collect();
        self
    }
}

/// # [Service]
/// Units to [Stop|Mask|Restart] while Rwfus is running
///
/// | Name             | Default                        |
/// |------------------|--------------------------------|
/// | Stop_Units       | var-cache-pacman.mount         |
/// | Mask_Units       | pacman-cleanup.service         |
/// | Restart_Units    | usr-local.mount polkit.service |
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Service {
    mask_units: Vec<String>,
    restart_units: Vec<String>,
    stop_units: Vec<String>,
}

impl Default for Service {
    fn default() -> Self {
        Service {
            mask_units: vec![String::from("pacman-cleanup.service")],
            restart_units: vec![
                String::from("usr-local.mount"),
                String::from("polkit.service"),
            ],
            stop_units: vec![String::from("var-cache-pacman.mount")],
        }
    }
}

impl<T: AsRef<Path>> From<T> for Service {
    fn from(_value: T) -> Self {
        Service::default()
    }
}

impl Service {
    pub fn mask_units<I, T>(mut self, services: T) -> Self
    where
        I: Into<String>,
        T: IntoIterator<Item = I>,
    {
        self.mask_units = cast_all(services);
        self
    }

    pub fn restart_units<I, T>(mut self, services: T) -> Self
    where
        I: Into<String>,
        T: IntoIterator<Item = I>,
    {
        self.restart_units = cast_all(services);
        self
    }

    pub fn stop_units<I, T>(mut self, services: T) -> Self
    where
        I: Into<String>,
        T: IntoIterator<Item = I>,
    {
        self.stop_units = cast_all(services);
        self
    }
}

/// OLD:
/// # [Overlay]
/// Where the overlayfs upperdirs and lowerdirs go
///
/// | Name             | Default                        |
/// |------------------|--------------------------------|
/// | Upper_Directory  | /opt/rwfus/mount/upper         |
/// | Work_Directory   | /opt/rwfus/mount/work          |
///  
/// NEW:
/// # [overlay]
///
/// | Name             | Default                        |
/// |------------------|--------------------------------|
/// | overlay_directory| /opt/rwfus/mount               |
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Overlay {
    overlay_directory: PathBuf,
}

impl<T: AsRef<Path>> From<T> for Overlay {
    fn from(value: T) -> Self {
        Overlay::default().overlay_directory(value.as_ref().join("mount"))
    }
}

impl Overlay {
    pub fn overlay_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.overlay_directory = path.as_ref().to_path_buf();
        self
    }
}

/// [Disk]
/// # Btrfs mount options. Make sure you keep `loop`
///
/// | Name             | Default                        |
/// |------------------|--------------------------------|
/// | Mount_Options    | loop,compress                  |
/// | Mount_Directory  | /opt/rwfus/mount               |
/// | Disk_Image_Path  | /opt/rwfus/rwfus.btrfs         |
/// | Disk_Image_Size  | 8G                             |
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Disk {
    disk_image_path: PathBuf,
    disk_image_size: String,
    mount_directory: PathBuf,
    mount_options: Vec<String>,
}

impl<T: AsRef<Path>> From<T> for Disk {
    fn from(value: T) -> Self {
        Disk::default()
            .disk_image_path(value.as_ref().join("rwfus.btrfs"))
            .mount_directory(value.as_ref().join("mount"))
    }
}

impl Default for Disk {
    fn default() -> Self {
        let path = Path::new("");
        Disk {
            disk_image_path: path.join("/opt/rwfus/rwfus.btrfs"),
            disk_image_size: "8G".to_owned(),
            mount_directory: path.join("/opt/rwfus/mount"),
            mount_options: cast_all(vec!["loop", "compress"]),
        }
    }
}

impl Disk {
    pub fn disk_image_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.disk_image_path = path.as_ref().to_path_buf();
        self
    }
    ///# FIXME: Stringly typed API
    pub fn disk_image_size(mut self, size: &str) -> Self {
        self.disk_image_size = size.to_owned();
        self
    }
    pub fn mount_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.mount_directory = path.as_ref().to_path_buf();
        self
    }
    pub fn mount_options<I, T>(mut self, options: T) -> Self
    where
        I: Into<String>,
        T: IntoIterator<Item = I>,
    {
        self.mount_options = cast_all(options);
        self
    }
}

/// [Configurator]
/// # Where systemd expects units to be
/// Systemd_Directory  /etc/systemd/system
/// # Storage directory for the daemon script and service-unit
/// Service_Directory /opt/rwfus/service
/// Install_Directory /home/.steamos/offload/usr/local/bin
/// # The path to the logfile
/// #Logfile           /var/log/rwfus.log
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Configurator {
    install_directory: PathBuf,
    service_directory: PathBuf,
    systemd_directory: PathBuf,
    logfile: PathBuf,
}

impl<T: AsRef<Path>> From<T> for Configurator {
    fn from(value: T) -> Self {
        Configurator::default()
            .install_directory(value.as_ref().join("rwfus.btrfs"))
            .service_directory(value.as_ref().join("service"))
    }
}

impl Default for Configurator {
    fn default() -> Self {
        let path = Path::new("");
        Configurator {
            install_directory: path.join("/home/.steamos/offload/usr/local/bin"),
            logfile: path.join("/var/log/rwfus.log"),
            service_directory: path.join("/opt/rwfus/service"),
            systemd_directory: path.join("/etc/systemd/system"),
        }
    }
}

impl Configurator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn install_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.install_directory = path.as_ref().to_path_buf();
        self
    }
    pub fn logfile<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.logfile = path.as_ref().to_path_buf();
        self
    }
    pub fn service_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.service_directory = path.as_ref().to_path_buf();
        self
    }
    pub fn systemd_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.systemd_directory = path.as_ref().to_path_buf();
        self
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Config {
    pub common: Common,
    pub configurator: Configurator,
    pub disk: Disk,
    pub overlay: Overlay,
    pub service: Service,
}

impl<T: AsRef<Path>> From<T> for Config {
    fn from(value: T) -> Self {
        Config {
            common: Common::from(&value),
            configurator: Configurator::from(&value),
            disk: Disk::from(&value),
            overlay: Overlay::from(&value),
            service: Service::from(&value),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let _path = Path::new("/opt/rwfus");
        Config {
            common: Common::default(),
            configurator: Configurator::default(),
            disk: Disk::default(),
            overlay: Overlay::default(),
            service: Service::default(),
        }
    }
}

pub mod io {
    use super::Config;
    use crate::{Error::ArgumentError, Result};
    use std::{
        fs::{read_to_string, File},
        io::prelude::*,
        path::Path,
    };

    /// Represents the valid config formats (serde serializers).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum ConfigFormat {
        Dummy,
        #[cfg(feature = "json")]
        Json,
        #[cfg(feature = "ron")]
        Ron,
        #[cfg(feature = "toml")]
        Toml,
        #[cfg(feature = "yaml")]
        Yaml,
    }

    impl Config {
        pub fn load<'a, P>(path: P, format: ConfigFormat) -> Result<Self>
        where
            P: AsRef<Path>,
        {
            use ConfigFormat::*;
            let file = read_to_string(path)?;
            match format {
                #[cfg(feature = "json")]
                Json => Ok(serde_json::from_str(&file)?),
                #[cfg(feature = "ron")]
                Ron => Ok(ron::from_str(&file)?),
                #[cfg(feature = "toml")]
                Toml => Ok(toml::from_str(&file)?),
                #[cfg(feature = "yaml")]
                Yaml => Ok(serde_yaml::from_str(&file)?),
                _ => Err(ArgumentError("".to_owned())),
            }
        }
        /// Saves `Config` to a file at `path`, in format `format`.
        pub fn save<P>(&self, path: P, format: ConfigFormat) -> Result<()>
        where
            P: AsRef<Path>,
        {
            let mut file = File::create(path)?;
            Ok(write!(file, "{}\n", self.to_string_pretty(format)?)?)
        }

        /// Deserializes `Config` to a pretty string.
        pub fn to_string_pretty(&self, format: ConfigFormat) -> Result<String> {
            use ConfigFormat::*;
            Ok(match format {
                #[cfg(feature = "json")]
                Json => serde_json::ser::to_string_pretty(&self)?,
                #[cfg(feature = "ron")]
                Ron => ron::ser::to_string_pretty(
                    &self,
                    ron::ser::PrettyConfig::default().struct_names(true),
                )?,
                #[cfg(feature = "toml")]
                Toml => toml::ser::to_string_pretty(&self)?,
                #[cfg(feature = "yaml")]
                Yaml => serde_yaml::to_string(&self)?,
                _ => "".to_owned(),
            })
        }
    }
}

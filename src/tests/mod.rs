mod config {
    pub use crate::{config::*, *};

    fn save_load(path: &str, format: io::ConfigFormat) -> Result<()> {
        let config = Config::from("/opt/rover");
        config.save(path, format)?;
        let object_under_test = Config::load(path, format)?;
        std::fs::remove_file(path)?;
        assert_eq!(object_under_test, config);
        Ok(())
    }

    #[cfg(feature = "json")]
    mod json {
        use super::*;
        use io::ConfigFormat::Json;
        #[test]
        fn save_load() -> Result<()> {
            super::save_load("/tmp/testfile.json", Json)?;
            Ok(())
        }
    }
    #[cfg(feature = "ron")]
    mod ron {
        use super::*;
        use io::ConfigFormat::Ron;

        #[test]
        fn save_load() -> Result<()> {
            super::save_load("/tmp/testfile.ron", Ron)?;
            Ok(())
        }
    }
    #[cfg(feature = "toml")]
    mod toml {
        use super::*;
        use io::ConfigFormat::Toml;

        #[test]
        fn save_load() -> Result<()> {
            super::save_load("/tmp/testfile.toml", Toml)?;
            Ok(())
        }
    }
    #[cfg(feature = "yaml")]
    mod yaml {
        use super::*;
        use io::ConfigFormat::Yaml;

        #[test]
        fn save_load() -> Result<()> {
            super::save_load("/tmp/testfile.yaml", Yaml)?;
            Ok(())
        }
    }
}

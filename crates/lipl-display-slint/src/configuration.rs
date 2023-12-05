use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use tracing::Level;
use tracing_log::log::LevelFilter;

use crate::{constant, ErrorExtension};

#[serde_as]
#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) log_level: LevelFilter,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) tracing_level: Level,
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) log_dir: PathBuf,
}

impl Config {
    pub(crate) fn from_file<P>(p: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        std::fs::read_to_string(p)
            .err_into()
            .and_then(|s| toml::from_str(&s).err_into())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Trace,
            tracing_level: Level::TRACE,
            log_dir: constant::DEFAULT_LOG_DIR.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::Config;
    use tracing::Level;
    use tracing_log::log::LevelFilter;

    #[test]
    fn config() {
        let config_s =
            "log_level = \"trace\"\ntracing_level = \"trace\"\nlog_dir = \"/var/log/lipl\"";
        let config: Config = toml::from_str(config_s).unwrap();

        assert_eq!(config.log_dir, "/var/log/lipl".parse::<PathBuf>().unwrap());
        assert_eq!(config.log_level, LevelFilter::Trace);
        assert_eq!(config.tracing_level, Level::TRACE);
    }

    #[test]
    fn config_file() {
        let filename = "pkg/common/lipl.toml";
        let config = Config::from_file(filename).unwrap();
        assert_eq!(config.log_dir, "/var/log/lipl".parse::<PathBuf>().unwrap());
        assert_eq!(config.log_level, LevelFilter::Trace);
        assert_eq!(config.tracing_level, Level::TRACE);
    }
}

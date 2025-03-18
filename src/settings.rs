use std::{fmt::format, path::Path};

use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

use crate::modules::git::GitSettings;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub format: String,
    pub git: GitSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir().unwrap();
        let config_dir = config_dir.to_str();
        let config_file = format!("{}/lazer.toml", config_dir.unwrap());
        let settings = Config::builder()
            .add_source(File::with_name(&config_file))
            .build()?;

        settings.try_deserialize()
    }
}

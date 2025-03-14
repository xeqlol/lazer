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
        let settings = Config::builder()
            .add_source(File::with_name("/Users/d.nemkov/Fun/lazer/lazer.toml"))
            .build()?;

        settings.try_deserialize()
    }
}

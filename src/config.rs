use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub weight: Option<f64>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "quote")]
    pub quotes: Vec<Quote>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config: Option<Config> = None;

        if let Ok(mut path) = current_dir() {
            path.push("thus_saith.toml");
            if let Some(result) = Self::load_file(&path) {
                config = Some(result?);
            };
        }

        if config.is_none() {
            if let Some(mut path) = dirs::home_dir() {
                path.push(".thus_saith.toml");
                if let Some(result) = Self::load_file(&path) {
                    config = Some(result?);
                };
            }
        }

        if config.is_none() {
            if let Some(mut path) = dirs::config_dir() {
                path.push("thus_saith/config.toml");
                if let Some(result) = Self::load_file(&path) {
                    config = Some(result?);
                };
            }
        }

        match config {
            Some(config) => Ok(config),
            None => {
                let default = include_str!("../config/default.toml");
                let context = "\
                    failed to parse the default configuration, \
                    please consider fixing 'config/default.toml' \
                    in the source code and recompiling the program";
                toml::from_str(default).context(context)
            }
        }
    }

    pub fn load_file(path: &Path) -> Option<Result<Self>> {
        if !path.exists() {
            return None;
        }
        if !path.is_file() {
            let error = anyhow!("'{}' exists, but is not a file", path.display());
            return Some(Err(error));
        }
        match read_to_string(path) {
            Err(error) => {
                let context = format!("failed to read '{}'", path.display());
                Some(Err(error).context(context))
            }
            Ok(content) => {
                let context = format!("failed to parse '{}'", path.display());
                Some(toml::from_str(&content).context(context))
            }
        }
    }
}

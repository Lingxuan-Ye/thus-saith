use anyhow::{Context, Result};
use owo_colors::{OwoColorize, Stream};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pace: Pace,
    pub messages: Messages,
    #[serde(rename = "quote")]
    pub quotes: Vec<RawQuote>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pace {
    pub mean: f64,
    pub stddev: f64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Messages {
    pub interrupt: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawQuote {
    pub weight: Option<f64>,
    pub content: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::load_default()?;

        if let Some(mut path) = dirs::config_dir() {
            path.push("thus-saith/config.toml");
            if path.exists() {
                config.patch_from(&path)?;
            }
        }

        if let Ok(mut path) = env::current_dir() {
            path.push("thus-saith.toml");
            if path.exists() {
                config.patch_from(&path)?;
            }
        }

        Ok(config)
    }

    pub fn load_default() -> Result<Self> {
        const DEFAULT: &str = include_str!("../config/default.toml");

        toml::from_str(DEFAULT).with_context(|| {
            format!(
                "\
                failed to parse the default configuration,  \
                please consider fixing '{}' in the project root \
                and recompiling the program\
                ",
                Path::new("config/default.toml")
                    .display()
                    .if_supports_color(Stream::Stderr, |text| text.blue())
            )
        })
    }

    pub fn patch_from(&mut self, path: &Path) -> Result<&mut Self> {
        let path_repr = path.display();
        let path_repr = path_repr.if_supports_color(Stream::Stderr, |text| text.blue());

        let string =
            fs::read_to_string(path).with_context(|| format!("failed to read '{path_repr}'"))?;
        let patch =
            toml::from_str(&string).with_context(|| format!("failed to parse '{path_repr}'"))?;

        self.patch(patch);

        Ok(self)
    }

    fn patch(&mut self, patch: ConfigPatch) -> &mut Self {
        if let Some(pace) = patch.pace {
            if let Some(mean) = pace.mean {
                self.pace.mean = mean;
            }
            if let Some(stddev) = pace.stddev {
                self.pace.stddev = stddev;
            }
        }

        if let Some(messages) = patch.messages
            && let Some(interrupt) = messages.interrupt
        {
            self.messages.interrupt = interrupt;
        }

        if let Some(quotes) = patch.quotes {
            self.quotes = quotes;
        }

        self
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigPatch {
    pace: Option<PacePatch>,
    messages: Option<MessagesPatch>,
    #[serde(rename = "quote")]
    quotes: Option<Vec<RawQuote>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PacePatch {
    mean: Option<f64>,
    stddev: Option<f64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MessagesPatch {
    interrupt: Option<String>,
}

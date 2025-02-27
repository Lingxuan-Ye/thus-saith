use anyhow::{ensure, Context, Result};
use dirs::config_dir;
use serde::Deserialize;
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RawMessages {
    interrupt: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawQuote {
    weight: Option<f64>,
    content: String,
}

#[derive(Debug, Deserialize)]
struct RaWConfig {
    messages: Option<RawMessages>,

    #[serde(rename = "quote")]
    quotes: Option<Vec<RawQuote>>,
}

pub struct Messages {
    pub interrupt: String,
}

pub struct Quote {
    weight: f64,
    pub content: String,
}

impl Quote {
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

pub struct Config {
    pub messages: Messages,

    /// # Guarantees
    ///
    /// - Non-empty.
    /// - All quotes have a positive finite weight.
    /// - The sum of all weights is finite.
    pub quotes: Vec<Quote>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::load_default()?;

        if let Some(mut path) = config_dir() {
            path.push("thus-saith/config.toml");
            if path.exists() {
                config.update_from_file(&path)?;
            }
        }

        if let Ok(mut path) = current_dir() {
            path.push("thus-saith.toml");
            if path.exists() {
                config.update_from_file(&path)?;
            }
        }

        Ok(config)
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let mut config = Config::load()?;
        config.update_from_file(path)?;
        Ok(config)
    }

    pub fn load_default() -> Result<Self> {
        let default = include_str!("../config/default.toml");
        let context = "\
            failed to parse the default configuration, \
            please consider fixing 'config/default.toml' \
            in the source code and recompiling the program";

        let raw_config: RaWConfig = toml::from_str(default).context(context)?;

        let raw_messages = raw_config.messages.context(context)?;
        let messages = Messages {
            interrupt: raw_messages.interrupt.context(context)?,
        };

        let raw_quotes = raw_config.quotes.context(context)?;
        let quotes = Config::normalize_quotes(raw_quotes)?;

        Ok(Config { messages, quotes })
    }

    fn update_from_file(&mut self, path: &Path) -> Result<&mut Self> {
        ensure!(path.is_file(), "'{}' is not a file", path.display());

        let string =
            read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;

        let config = toml::from_str(&string)
            .with_context(|| format!("failed to parse '{}'", path.display()))?;

        self.update(config)
            .with_context(|| format!("failed to normalize quotes from '{}'", path.display()))
    }

    fn update(&mut self, config: RaWConfig) -> Result<&mut Self> {
        if let Some(messages) = config.messages {
            if let Some(interrupt) = messages.interrupt {
                self.messages.interrupt = interrupt;
            }
        }

        if let Some(quotes) = config.quotes {
            self.quotes = Config::normalize_quotes(quotes)?;
        }

        Ok(self)
    }

    fn normalize_quotes(quotes: Vec<RawQuote>) -> Result<Vec<Quote>> {
        let mut normalized = Vec::with_capacity(quotes.len());
        let mut unweighted = Vec::new();
        let mut total_weight = 0.0;

        for quote in quotes {
            match quote.weight {
                None => {
                    unweighted.push(quote);
                }
                Some(weight) => {
                    if weight == 0.0
                        || weight.is_nan()
                        || weight.is_infinite()
                        || weight.is_sign_negative()
                    {
                        continue;
                    }
                    total_weight += weight;
                    let content = quote.content;
                    normalized.push(Quote { weight, content });
                }
            }
        }

        if !unweighted.is_empty() {
            let weight = if normalized.is_empty() {
                1.0
            } else {
                // The maximum length of the input `Vec<RawQuote>` in current
                // implementation is `isize::MAX / 40`, which in 64-bit systems
                // exceeds the maximum consecutive integer that a `f64` can
                // represent (2^53 - 1). In that case, the divisor can become
                // inaccurate. However, this may be pointless since the system
                // should have already run out of memory.
                total_weight / normalized.len() as f64
            };
            for quote in unweighted {
                total_weight += weight;
                let content = quote.content;
                normalized.push(Quote { weight, content });
            }
        }

        ensure!(total_weight != 0.0, "no valid quotes found");
        ensure!(total_weight.is_finite(), "total weight overflows");

        Ok(normalized)
    }
}

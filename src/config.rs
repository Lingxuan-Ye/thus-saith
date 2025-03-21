use anyhow::{Context, Error, Result, ensure};
use rand::prelude::*;
use rand_distr::weighted::WeightedIndex;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

pub struct Config {
    pub messages: Messages,
    pub quotes: QuotePool,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::load_default()?;

        if let Some(mut path) = dirs::config_dir() {
            path.push("thus-saith/config.toml");
            if path.exists() {
                config.update(&path)?;
            }
        }

        if let Ok(mut path) = env::current_dir() {
            path.push("thus-saith.toml");
            if path.exists() {
                config.update(&path)?;
            }
        }

        Ok(config)
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        let mut config = Config::load()?;
        config.update(path)?;
        Ok(config)
    }

    fn load_default() -> Result<Self> {
        let default = include_str!("../config/default.toml");
        let context = "\
            failed to parse the default configuration, \
            please consider fixing 'config/default.toml' \
            in the source code and recompiling the program";

        let config: RaWConfig = toml::from_str(default).context(context)?;

        let interrupt = config
            .messages
            .and_then(|messages| messages.interrupt)
            .context(context)?;
        let messages = Messages { interrupt };

        let quotes = config
            .quotes
            .context(context)?
            .try_into()
            .context(context)?;

        Ok(Config { messages, quotes })
    }

    fn update(&mut self, path: &Path) -> Result<&mut Self> {
        let path_repr = path.display();

        ensure!(path.is_file(), "'{}' is not a file", path_repr);
        let string =
            fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path_repr))?;
        let config: RaWConfig =
            toml::from_str(&string).with_context(|| format!("failed to parse '{}'", path_repr))?;

        if let Some(messages) = config.messages {
            if let Some(interrupt) = messages.interrupt {
                self.messages.interrupt = interrupt;
            }
        }

        if let Some(quotes) = config.quotes {
            self.quotes = quotes
                .try_into()
                .with_context(|| format!("failed to normalize quotes from '{}'", path_repr))?;
        }

        Ok(self)
    }
}

pub struct Messages {
    pub interrupt: String,
}

struct Quote {
    weight: f64,
    content: String,
}

/// # Guarantees
///
/// - Non-empty.
/// - All quotes have a positive finite weight.
/// - The sum of all weights is finite.
pub struct QuotePool(Vec<Quote>);

impl QuotePool {
    pub fn sample(&self) -> &str {
        let weights = self.0.iter().map(|quote| quote.weight);
        let Ok(distribution) = WeightedIndex::new(weights) else {
            unreachable!()
        };
        let index = rand::rng().sample(distribution);
        &self.0[index].content
    }
}

impl TryFrom<Vec<RawQuote>> for QuotePool {
    type Error = Error;

    fn try_from(value: Vec<RawQuote>) -> Result<Self> {
        let mut normalized = Vec::with_capacity(value.len());
        let mut unweighted = Vec::new();
        let mut total_weight = 0.0;

        for quote in value {
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

        Ok(Self(normalized))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMessages {
    interrupt: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawQuote {
    weight: Option<f64>,
    content: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RaWConfig {
    messages: Option<RawMessages>,

    #[serde(rename = "quote")]
    quotes: Option<Vec<RawQuote>>,
}

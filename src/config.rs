use anyhow::{Context, Error, Result, ensure};
use owo_colors::{OwoColorize, Stream};
use rand::prelude::*;
use rand_distr::weighted::WeightedIndex;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

pub struct Config {
    pub distribution: Distribution,
    pub messages: Messages,
    pub quotes: QuotePool,
}

pub struct Distribution {
    mean: f64,
    stddev: f64,
}

pub struct Messages {
    pub interrupt: String,
}

/// # Guarantees
///
/// - Non-empty.
/// - All quotes have a positive finite weight.
/// - The sum of all weights is finite.
pub struct QuotePool(Vec<Quote>);

struct Quote {
    weight: f64,
    content: String,
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
        const DEFAULT: &str = include_str!("../config/default.toml");
        const CONTEXT: &str = "\
            failed to parse the default configuration, \
            please consider fixing 'config/default.toml' \
            in the source code and recompiling the program";

        let config: RawConfig = toml::from_str(DEFAULT).context(CONTEXT)?;

        let distribution = config.distribution.context(CONTEXT)?;
        let distribution = Distribution {
            mean: distribution.mean.context(CONTEXT)?,
            stddev: distribution.stddev.context(CONTEXT)?,
        };

        let messages = config.messages.context(CONTEXT)?;
        let messages = Messages {
            interrupt: messages.interrupt.context(CONTEXT)?,
        };

        let quotes = config.quotes.context(CONTEXT)?;
        let quotes = quotes.try_into().context(CONTEXT)?;

        Ok(Config {
            distribution,
            messages,
            quotes,
        })
    }

    fn update(&mut self, path: &Path) -> Result<&mut Self> {
        let path_repr = path.display();

        ensure!(path.is_file(), "'{}' is not a file", path_repr);
        let string =
            fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path_repr))?;
        let config: RawConfig =
            toml::from_str(&string).with_context(|| format!("failed to parse '{}'", path_repr))?;

        if let Some(distribution) = config.distribution {
            self.distribution.update(distribution);
        }
        if let Some(messages) = config.messages {
            self.messages.update(messages);
        }
        if let Some(quotes) = config.quotes {
            self.quotes = quotes
                .try_into()
                .with_context(|| format!("failed to normalize quotes from '{}'", path_repr))?;
        }

        Ok(self)
    }
}

impl Distribution {
    pub fn mean(&self) -> Result<f64> {
        ensure!(
            !self.mean.is_nan() && self.mean.is_finite() && self.mean > 0.0,
            "'{}' must be positive",
            "mean".if_supports_color(Stream::Stderr, |text| text.yellow())
        );
        Ok(self.mean)
    }

    pub fn stddev(&self) -> Result<f64> {
        ensure!(
            !self.stddev.is_nan() && self.stddev.is_finite() && self.stddev >= 0.0,
            "'{}' must be non-negative",
            "stddev".if_supports_color(Stream::Stderr, |text| text.yellow())
        );
        Ok(self.stddev)
    }

    fn update(&mut self, distribution: RawDistribution) -> &mut Self {
        if let Some(mean) = distribution.mean {
            self.mean = mean;
        }
        if let Some(stddev) = distribution.stddev {
            self.stddev = stddev;
        }
        self
    }
}

impl Messages {
    fn update(&mut self, messages: RawMessages) -> &mut Self {
        if let Some(interrupt) = messages.interrupt {
            self.interrupt = interrupt;
        }
        self
    }
}

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
struct RawConfig {
    distribution: Option<RawDistribution>,
    messages: Option<RawMessages>,
    #[serde(rename = "quote")]
    quotes: Option<Vec<RawQuote>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDistribution {
    mean: Option<f64>,
    stddev: Option<f64>,
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

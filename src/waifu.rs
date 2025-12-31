use anyhow::{Context, Error, Result, ensure};
use owo_colors::{OwoColorize, Stream};
use rand::prelude::*;
use rand_distr::LogNormal;
use std::fmt::Display;
use std::io::Write;
use std::time::{Duration, Instant};

pub struct Waifu {
    /// The distribution of the milliseconds taken per character.
    distr: LogNormal<f64>,
    rng: ThreadRng,
}

impl Waifu {
    pub fn new(mean: Mean, stddev: Stddev) -> Result<Self> {
        let mean = mean.0;
        let stddev = stddev.0;
        let variance = (stddev.powi(2) / mean.powi(2) + 1.0).ln();
        ensure!(variance.is_finite(), "calculation overflows");
        let mu = mean.ln() - 0.5 * variance;
        let sigma = variance.sqrt();
        let distr = LogNormal::new(mu, sigma).context("unexpected error")?;
        let rng = rand::rng();
        Ok(Self { distr, rng })
    }

    pub fn say<S, T, W>(&mut self, stream: S, mut output: W) -> Result<&mut Self>
    where
        S: IntoIterator<Item = T>,
        T: Display,
        W: Write,
    {
        for token in stream {
            let sampled = self.rng.sample(self.distr);
            let duration = Duration::from_secs_f64(sampled / 1000.0);
            let start = Instant::now();
            while start.elapsed() < duration {}
            write!(output, "{token}")?;
            output.flush()?;
        }
        writeln!(output)?;
        Ok(self)
    }
}

pub struct Mean(f64);

impl TryFrom<f64> for Mean {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self> {
        ensure!(
            !value.is_nan() && value.is_finite() && value > 0.0,
            "'{}' must be finite positive",
            "mean".if_supports_color(Stream::Stderr, |text| text.yellow())
        );

        Ok(Self(value))
    }
}

pub struct Stddev(f64);

impl TryFrom<f64> for Stddev {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self> {
        ensure!(
            !value.is_nan() && value.is_finite() && value >= 0.0,
            "'{}' must be finite non-negative",
            "stddev".if_supports_color(Stream::Stderr, |text| text.yellow())
        );

        Ok(Self(value))
    }
}

use crate::config::Pace;
use anyhow::{Context, Result, ensure};
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
    pub fn with_pace(pace: Pace) -> Result<Self> {
        let mean = pace.mean()?;
        let stddev = pace.stddev()?;
        let variance = (stddev.powi(2) / mean.powi(2) + 1.0).ln();
        ensure!(variance.is_finite(), "calculation overflows");
        let mu = mean.ln() - 0.5 * variance;
        let sigma = variance.sqrt();
        let distr = LogNormal::new(mu, sigma).context("unexpected error")?;
        let rng = rand::rng();
        Ok(Self { distr, rng })
    }

    pub fn say<W, I, T>(&mut self, mut output: W, tokens: I) -> Result<&mut Self>
    where
        W: Write,
        I: IntoIterator<Item = T>,
        T: Display,
    {
        for token in tokens {
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

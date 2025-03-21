use ansi_term::Colour::Yellow;
use anyhow::{Context, Result, ensure};
use rand::prelude::*;
use rand_distr::LogNormal;
use std::fmt::Display;
use std::io::Write;
use std::time::{Duration, Instant};

pub struct Typist {
    /// The distribution of the milliseconds taken per character.
    distribution: LogNormal<f64>,
    rng: ThreadRng,
}

impl Typist {
    pub fn with_millis_per_char(mean: f64, std_dev: f64) -> Result<Self> {
        Self::sanity_check(mean, std_dev)?;
        let variance = (std_dev.powi(2) / mean.powi(2) + 1.0).ln();
        ensure!(variance.is_finite(), "calculation overflows");
        let mu = mean.ln() - 0.5 * variance;
        let sigma = variance.sqrt();
        let distribution = LogNormal::new(mu, sigma).context("unexpected error")?;
        let rng = rand::rng();
        Ok(Self { distribution, rng })
    }

    #[allow(dead_code)]
    pub fn with_chars_per_min(mean: f64, std_dev: f64) -> Result<Self> {
        Self::sanity_check(mean, std_dev)?;
        // Pretty much sure that the formula is mathematically correct.
        // However, as the `std_dev` increases, the resulting `mean`
        // deviates from the theoretical value. I have no idea what is
        // going on here, maybe it just because I'm not good at math.
        let variance = (std_dev.powi(2) / mean.powi(2) + 1.0).ln();
        ensure!(variance.is_finite(), "calculation overflows");
        let mu = -(mean / 60000.0).ln() + 0.5 * variance;
        let sigma = variance.sqrt();
        let distribution = LogNormal::new(mu, sigma).context("unexpected error")?;
        let rng = rand::rng();
        Ok(Self { distribution, rng })
    }

    /// In this context, a `char` means a valid Unicode character,
    /// with or without ANSI escape codes.
    pub fn type_out<C, I, W>(&mut self, chars: I, mut output: W) -> Result<&mut Self>
    where
        C: Display,
        I: IntoIterator<Item = C>,
        W: Write,
    {
        for char in chars {
            let sampled = self.rng.sample(self.distribution);
            let duration = Duration::from_secs_f64(sampled / 1000.0);
            let start = Instant::now();
            while start.elapsed() < duration {}
            write!(output, "{char}")?;
            output.flush()?;
        }
        Ok(self)
    }

    fn sanity_check(mean: f64, std_dev: f64) -> Result<()> {
        ensure!(
            !mean.is_nan() && mean.is_finite() && mean > 0.0,
            "'{}' must be positive",
            Yellow.paint("mean")
        );
        ensure!(
            !std_dev.is_nan() && std_dev.is_finite() && std_dev >= 0.0,
            "'{}' must be non-negative",
            Yellow.paint("std-dev")
        );
        Ok(())
    }
}

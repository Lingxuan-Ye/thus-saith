use crate::sanity::{NonNegFinite, PosFinite};
use anyhow::Result;
use rand::prelude::*;
use rand_distr::LogNormal;
use std::fmt::Display;
use std::io::Write;
use std::time::{Duration, Instant};

pub struct Typist {
    distribution: LogNormal<f64>,
    rng: ThreadRng,
}

impl Typist {
    pub fn with_chars_per_min(mean: PosFinite, std_dev: NonNegFinite) -> Self {
        let mean = mean.value();
        let std_dev = std_dev.value();
        let sigma = (std_dev.powi(2) / mean.powi(2) + 1.0).ln().sqrt();
        let mu = mean.ln() - 0.5 * sigma.powi(2);
        let distribution = LogNormal::new(mu, sigma).expect("will never fail");
        let rng = thread_rng();
        Self { distribution, rng }
    }

    /// In this context, a `C` (char) means a valid Unicode character,
    /// with or without ANSI escape codes.
    pub fn type_out<C, I, W>(&mut self, quote: I, mut output: W) -> Result<&mut Self>
    where
        C: Display,
        I: IntoIterator<Item = C>,
        W: Write,
    {
        for char in quote {
            let chars_per_min = self.rng.sample(self.distribution);
            let secs_per_char = Duration::from_secs_f64(60.0 / chars_per_min);
            let start = Instant::now();
            while start.elapsed() < secs_per_char {}
            write!(output, "{char}")?;
            output.flush()?;
        }
        Ok(self)
    }
}

use crate::check::SanityCheck;
use crate::error::ValueError;
use rand::prelude::*;
use rand_distr::LogNormal;
use std::borrow::Cow;
use std::io::{Error as IoError, Write};
use std::time::{Duration, Instant};

/// In this context, a `Char` is a valid Unicode character,
/// with or without ANSI escape codes.
pub type Char<'a> = Cow<'a, str>;

#[derive(Debug)]
pub struct Typist {
    distribution: LogNormal<f64>,
    rng: ThreadRng,
}

impl Typist {
    pub fn with_chars_per_min(mean: f64, std_dev: f64) -> Result<Self, ValueError> {
        let mean = mean
            .ensure_non_nan()?
            .ensure_non_zero()?
            .ensure_positive()?
            .ensure_finite()?;
        let std_dev = std_dev
            .ensure_non_nan()?
            .ensure_non_zero()?
            .ensure_positive()?
            .ensure_finite()?;
        let sigma = (std_dev.powi(2) / mean.powi(2) + 1.0).ln().sqrt();
        let mu = mean.ln() - 0.5 * sigma.powi(2);
        let distribution = LogNormal::new(mu, sigma).expect("will never fail");
        let rng = thread_rng();
        Ok(Self { distribution, rng })
    }

    pub fn type_out<'a, I, W>(&mut self, quote: I, mut output: W) -> Result<&mut Self, IoError>
    where
        I: IntoIterator<Item = Char<'a>>,
        W: Write,
    {
        for char in quote {
            let chars_per_min = self.rng.sample(self.distribution);
            let secs_per_char = Duration::from_secs_f64(60.0 / chars_per_min);
            let start = Instant::now();
            while start.elapsed() < secs_per_char {}
            write!(output, "{char}")?;
        }
        Ok(self)
    }
}

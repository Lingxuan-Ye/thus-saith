use crate::config::Quote;
use anyhow::{Context, Result};
use rand::prelude::*;
use rand_distr::WeightedIndex;

pub(crate) struct Selector;

impl Selector {
    pub(crate) fn select(quotes: &[Quote]) -> Result<&str> {
        let weights = quotes.iter().map(|quote| quote.weight());
        let distribution = WeightedIndex::new(weights).context("unexpected error")?;
        let index = thread_rng().sample(distribution);
        Ok(&quotes[index].content)
    }
}

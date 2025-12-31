use crate::config::RawQuote;
use anyhow::{Error, Result, ensure};
use rand::prelude::*;
use rand_distr::weighted::WeightedIndex;

/// # Invariants
///
/// - Non-empty.
/// - All quotes have a positive finite weight.
/// - The sum of all weights is finite.
pub struct QuotePool(Vec<Quote>);

struct Quote {
    weight: f64,
    content: String,
}

impl QuotePool {
    pub fn choose(&self) -> &str {
        let weights = self.0.iter().map(|quote| quote.weight);
        let Ok(distr) = WeightedIndex::new(weights) else {
            unreachable!()
        };
        let index = rand::rng().sample(distr);
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

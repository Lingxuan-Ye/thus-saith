use crate::config::{Config, Quote};
use rand::Rng;
use rand_distr::WeightedIndex;

struct NormalizedQuote<'a> {
    weight: f64,
    content: &'a str,
}

pub struct Selector;

impl Selector {
    pub fn select(config: &Config) -> Option<&str> {
        let normalized = Self::normalize(&config.quotes);
        let distribution = WeightedIndex::new(normalized.iter().map(|quote| quote.weight)).ok()?;
        let mut rng = rand::thread_rng();
        let index = rng.sample(distribution);
        Some(normalized[index].content)
    }

    fn normalize(quotes: &Vec<Quote>) -> Vec<NormalizedQuote<'_>> {
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
                    normalized.push(NormalizedQuote {
                        weight,
                        content: &quote.content,
                    });
                }
            }
        }
        if !unweighted.is_empty() {
            let average_weight = total_weight / normalized.len() as f64;
            for quote in unweighted {
                normalized.push(NormalizedQuote {
                    weight: average_weight,
                    content: &quote.content,
                });
            }
        }
        normalized
    }
}

use serde::Deserialize;

static DEFAULT: &str = include_str!("../config/default.toml");

#[derive(Debug, Deserialize)]
pub struct Quote {
    pub weight: Option<f64>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "quote")]
    pub quotes: Vec<Quote>,
}

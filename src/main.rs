use self::cli::Args;
use self::config::Config;
use self::quote_pool::QuotePool;
use self::signal::set_handler;
use self::tokenizer::Tokenizer;
use self::waifu::{Mean, Stddev, Waifu};
use anyhow::Result;
use eoe::ExitOnError;
use std::io::stdout;

mod cli;
mod config;
mod quote_pool;
mod signal;
mod tokenizer;
mod waifu;

fn run() -> Result<()> {
    let args = Args::parse();
    let mut config = Config::load()?;

    if let Some(path) = args.config {
        config.patch_from(path)?;
    }

    if let Some(mean) = args.mean {
        config.pace.mean = mean;
    }

    if let Some(stddev) = args.stddev {
        config.pace.stddev = stddev;
    }

    set_handler(config.message)?;

    let mean = Mean::try_from(config.pace.mean)?;
    let stddev = Stddev::try_from(config.pace.stddev)?;
    let pool = QuotePool::try_from(config.quotes)?;
    let quote = pool.choose();
    let stream = Tokenizer::tokenize(quote);
    let output = stdout();

    Waifu::new(mean, stddev)?.say(stream, output)?;

    Ok(())
}

fn main() {
    run().exit_on_error();
}

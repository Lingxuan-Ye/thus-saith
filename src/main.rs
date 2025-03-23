use self::config::Config;
use self::signal::set_handler;
use self::tokenizer::Tokenizer;
use self::typist::Typist;
use anyhow::Result;
use eoe::ExitOnError;
use std::io::stdout;

mod cli;
mod config;
mod signal;
mod tokenizer;
mod typist;

fn execute() -> Result<()> {
    let config = Config::load()?;

    set_handler(config.messages)?;

    let quote = config.quotes.choose();
    let chars = Tokenizer::tokenize(quote);
    let output = stdout();

    let pace = config.pace;
    let mean = pace.mean()?;
    let stddev = pace.stddev()?;
    Typist::with_millis_per_char(mean, stddev)?.type_out(chars, output)?;

    Ok(())
}

fn main() {
    execute().exit_on_error();
}

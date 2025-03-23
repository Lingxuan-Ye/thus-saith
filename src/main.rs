use self::config::Config;
use self::signal::set_handler;
use self::tokenizer::Tokenizer;
use self::waifu::Waifu;
use anyhow::Result;
use eoe::ExitOnError;
use std::io::stdout;

mod cli;
mod config;
mod signal;
mod tokenizer;
mod waifu;

fn execute() -> Result<()> {
    let config = Config::load()?;

    set_handler(config.messages)?;

    let output = stdout();
    let quote = config.quotes.choose();
    let tokens = Tokenizer::tokenize(quote);
    Waifu::with_pace(config.pace)?.say(output, tokens)?;

    Ok(())
}

fn main() {
    execute().exit_on_error();
}

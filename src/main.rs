use self::cli::{Args, build_command};
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
    let command = build_command();
    let matches = command.get_matches();
    let args = Args::from_matches(&matches);

    let config = match args.config {
        None => Config::load()?,
        Some(path) => Config::load_from(path)?,
    };

    set_handler(config.messages)?;

    let quote = config.quotes.sample();
    let chars = Tokenizer::tokenize(quote);
    let output = stdout();

    Typist::with_millis_per_char(args.mean, args.std_dev)?.type_out(chars, output)?;

    Ok(())
}

fn main() {
    execute().exit_on_error();
}

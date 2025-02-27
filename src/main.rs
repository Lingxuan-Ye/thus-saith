use self::cli::{Args, build_command};
use self::config::Config;
use self::select::Selector;
use self::signal::set_handler_for_sigint;
use self::tokenizer::Tokenizer;
use self::typist::Typist;
use anyhow::Result;
use eoe::ExitOnError;
use std::io::stdout;

mod cli;
mod config;
mod select;
mod signal;
mod tokenizer;
mod typist;

fn execute() -> Result<()> {
    let command = build_command();
    let matches = command.get_matches();
    let args = Args::from_matches(&matches);

    let config = match args.config {
        Some(file) => Config::load_from_file(file)?,
        None => Config::load()?,
    };

    set_handler_for_sigint(config.messages.interrupt);

    let mut typist = Typist::with_millis_per_char(args.mean, args.std_dev)?;

    let quote = Selector::select(&config.quotes)?;
    let chars = Tokenizer::tokenize(quote);
    let output = stdout();
    typist.type_out(chars, output)?;

    Ok(())
}

fn main() {
    execute().exit_on_error();
}

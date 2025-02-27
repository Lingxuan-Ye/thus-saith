use anyhow::Result;
use cli::Cli;
use config::Config;
use eoe::ExitOnError;
use select::Selector;
use signal::set_handler_for_sigint;
use std::io::stdout;
use std::path::PathBuf;
use tokenizer::Tokenizer;
use typist::Typist;

mod cli;
mod config;
mod select;
mod signal;
mod tokenizer;
mod typist;

fn execute() -> Result<()> {
    let matches = Cli::new().get_matches();

    let file: Option<&PathBuf> = matches.get_one("config");
    let config = match file {
        Some(file) => Config::load_from_file(file)?,
        None => Config::load()?,
    };

    set_handler_for_sigint(config.messages.interrupt);

    let Some(&mean): Option<&f64> = matches.get_one("mean") else {
        unreachable!()
    };
    let Some(&std_dev): Option<&f64> = matches.get_one("std-dev") else {
        unreachable!()
    };
    let mut typist = Typist::with_millis_per_char(mean, std_dev)?;

    let quote = Selector::select(&config.quotes)?;
    let chars = Tokenizer::tokenize(quote);
    let output = stdout();
    typist.type_out(chars, output)?;

    Ok(())
}

fn main() {
    execute().exit_on_error();
}

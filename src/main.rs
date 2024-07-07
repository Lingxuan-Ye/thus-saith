use cli::Cli;
use config::Config;
use exit::{handle_interrupt, ExitOnError};
use sanity::{NonNegFinite, PosFinite};
use select::Selector;
use std::io::stdout;
use tokenizer::Tokenizer;
use typist::Typist;

mod cli;
mod config;
mod exit;
mod sanity;
mod select;
mod tokenizer;
mod typist;

fn main() {
    let config = Config::load().exit_on_error();

    handle_interrupt(config.messages.interrupt);

    let matches = Cli::new().get_matches();

    let mean: PosFinite = *matches.get_one("mean").expect("will never fail");
    let std_dev: NonNegFinite = *matches.get_one("std-dev").expect("will never fail");
    let mut typist = Typist::with_chars_per_min(mean, std_dev);

    let quote = Selector::select(&config.quotes).exit_on_error();
    let chars = Tokenizer::tokenize(quote);
    let output = stdout();
    typist.type_out(chars, output).exit_on_error();
}

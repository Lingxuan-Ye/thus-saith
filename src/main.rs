use cli::Cli;
use config::Config;
use exit::{handle_interrupt, ExitOnError};
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

    let mean: f64 = *matches.get_one("mean").expect("will never fail");
    let std_dev: f64 = *matches.get_one("std-dev").expect("will never fail");
    let mut typist = Typist::with_millis_per_char(mean, std_dev).exit_on_error();

    let quote = Selector::select(&config.quotes).exit_on_error();
    let chars = Tokenizer::tokenize(quote);
    let output = stdout();
    typist.type_out(chars, output).exit_on_error();
}

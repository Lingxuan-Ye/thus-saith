use ansi_term::Colour::Yellow;
use anyhow::Context;
use eoe::ExitOnError;
use std::process::exit;

pub(crate) fn set_handler_for_sigint(message: String) {
    ctrlc::set_handler(move || {
        eprintln!();
        eprintln!();
        eprintln!("{}", Yellow.bold().paint(&message));
        exit(1);
    })
    .context("system error occurred")
    .exit_on_error();
}

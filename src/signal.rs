use super::config::Messages;
use ansi_term::Colour::Yellow;
use anyhow::{Context, Result};
use std::process::exit;

pub(crate) fn set_handler(message: Messages) -> Result<()> {
    ctrlc::set_handler(move || {
        eprintln!();
        eprintln!();
        eprintln!("{}", Yellow.bold().paint(&message.interrupt));
        exit(1);
    })
    .context("system error occurred")
}
